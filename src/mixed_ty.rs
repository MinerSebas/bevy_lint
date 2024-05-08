use either::Either;
use rustc_lint::LateContext;
use rustc_span::Span;

use crate::bevy_paths;

#[derive(Debug, Clone, Copy)]
pub(crate) struct MixedTy<'tcx> {
    pub(crate) hir: Either<&'tcx rustc_hir::Ty<'tcx>, Span>,
    pub(crate) middle: rustc_middle::ty::Ty<'tcx>,
}

impl<'tcx> MixedTy<'tcx> {
    pub(crate) fn fields_from_struct_item(
        ctx: &LateContext<'tcx>,
        item: &rustc_hir::Item<'tcx>,
    ) -> Option<Vec<Self>> {
        match item.kind {
            rustc_hir::ItemKind::Struct(
                rustc_hir::VariantData::Struct {
                    fields: hir_fields, ..
                }
                | rustc_hir::VariantData::Tuple(hir_fields, _, _),
                _,
            ) => {
                let middle: rustc_middle::ty::Ty =
                    ctx.tcx.type_of(item.owner_id.def_id).skip_binder();

                let middle_fields = match middle.kind() {
                    rustc_middle::ty::TyKind::Adt(def, _) => {
                        if !def.is_struct() {
                            return None;
                        }
                        def.variants()
                            .iter()
                            .flat_map(|variant| &variant.fields)
                            .map(|field_def| ctx.tcx.type_of(field_def.did))
                    }
                    _ => return None,
                };

                let vec = hir_fields
                    .iter()
                    .map(|hir_field| hir_field.ty)
                    .zip(middle_fields)
                    .map(|item| Self {
                        hir: Either::Left(item.0),
                        middle: item.1.skip_binder(),
                    })
                    .collect();
                Some(vec)
            }
            _ => None,
        }
    }

    pub(crate) fn fn_inputs_from_fn_item(
        ctx: &LateContext<'tcx>,
        item: &rustc_hir::Item<'tcx>,
    ) -> Option<Vec<Self>> {
        match item.kind {
            rustc_hir::ItemKind::Fn(rustc_hir::FnSig { decl, .. }, _, _) => {
                // rust-analyzer doesn't find `type_of`.
                // The return type is manually specified to still get autocompletion.
                let middle: rustc_middle::ty::Ty =
                    ctx.tcx.type_of(item.owner_id.def_id).skip_binder();

                let inputs = middle.fn_sig(ctx.tcx).skip_binder().inputs();

                let vec = decl
                    .inputs
                    .iter()
                    .zip(inputs.iter())
                    .map(|item| Self {
                        hir: Either::Left(item.0),
                        middle: *item.1,
                    })
                    .collect();

                Some(vec)
            }
            _ => None,
        }
    }

    pub(crate) fn fn_inputs_from_impl_fn_item(
        ctx: &LateContext<'tcx>,
        item: &rustc_hir::ImplItem<'tcx>,
    ) -> Option<Vec<Self>> {
        match item.kind {
            rustc_hir::ImplItemKind::Fn(rustc_hir::FnSig { decl, .. }, _) => {
                // rust-analyzer doesn't find `type_of`.
                // The return type is manually specified to still get autocompletion.
                let middle: rustc_middle::ty::Ty =
                    ctx.tcx.type_of(item.owner_id.def_id).skip_binder();

                let inputs = middle.fn_sig(ctx.tcx).skip_binder().inputs();

                let vec = decl
                    .inputs
                    .iter()
                    .zip(inputs.iter())
                    .map(|item| Self {
                        hir: Either::Left(item.0),
                        middle: *item.1,
                    })
                    .collect();

                Some(vec)
            }
            _ => None,
        }
    }

    pub(crate) fn fn_inputs_from_trait_fn_item(
        ctx: &LateContext<'tcx>,
        item: &rustc_hir::TraitItem<'tcx>,
    ) -> Option<Vec<Self>> {
        match item.kind {
            rustc_hir::TraitItemKind::Fn(rustc_hir::FnSig { decl, .. }, _) => {
                // rust-analyzer doesn't find `type_of`.
                // The return type is manually specified to still get autocompletion.
                let middle: rustc_middle::ty::Ty =
                    ctx.tcx.type_of(item.owner_id.def_id).skip_binder();

                let inputs = middle.fn_sig(ctx.tcx).skip_binder().inputs();

                let vec = decl
                    .inputs
                    .iter()
                    .zip(inputs.iter())
                    .map(|item| Self {
                        hir: Either::Left(item.0),
                        middle: *item.1,
                    })
                    .collect();

                Some(vec)
            }
            _ => None,
        }
    }

    pub(crate) fn extract_tuple_types(&self) -> Option<Vec<Self>> {
        let middle_types: Vec<_> = match self.middle.kind() {
            rustc_middle::ty::TyKind::Tuple(_) => self.middle.tuple_fields().iter().collect(),
            _ => return None,
        };

        let hir_data: Vec<_> = match self.hir {
            Either::Left(ty) => match ty.kind {
                rustc_hir::TyKind::Tup(hir_types) => hir_types.iter().map(Either::Left).collect(),
                _ => return None,
            },
            Either::Right(_) => std::iter::repeat(self.hir)
                .take(middle_types.len())
                .collect(),
        };

        assert_eq!(hir_data.len(), middle_types.len());

        let vec = hir_data
            .iter()
            .zip(middle_types)
            .map(|item| Self {
                hir: *item.0,
                middle: item.1,
            })
            .collect();
        Some(vec)
    }

    pub(crate) fn extract_generics_from_struct(&self) -> Option<Vec<Self>> {
        let middle_generics: Vec<_> = match self.middle.kind() {
            rustc_middle::ty::TyKind::Adt(_, generics) => generics
                .iter()
                .filter_map(|generic| {
                    if let rustc_middle::ty::GenericArgKind::Type(ty) = generic.unpack() {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .collect(),
            _ => return None,
        };

        let mut hir_generics: Vec<_> = match self.hir {
            Either::Left(ty) => match &ty.kind {
                rustc_hir::TyKind::Path(q_path) => clippy_utils::qpath_generic_tys(q_path)
                    .map(Either::Left)
                    .collect(),
                _ => return None,
            },
            Either::Right(_) => std::iter::repeat(self.hir)
                .take(middle_generics.len())
                .collect(),
        };

        // Fix for aliases
        if hir_generics.is_empty() && !middle_generics.is_empty() {
            hir_generics = std::iter::repeat(Either::Right(self.span()))
                .take(middle_generics.len())
                .collect();
        }

        let vec = hir_generics
            .iter()
            .zip(middle_generics.iter())
            .map(|item| Self {
                hir: *item.0,
                middle: *item.1,
            })
            .collect();

        Some(vec)
    }

    pub(crate) fn strip_reference(&self) -> Option<(Self, rustc_ast::Mutability)> {
        let hir_data = match self.hir {
            Either::Left(hir_ty) => match hir_ty.kind {
                rustc_hir::TyKind::Ref(_, rustc_hir::MutTy { ty, .. }) => Either::Left(ty),
                _ => return None,
            },
            Either::Right(_) => self.hir,
        };

        let middle_data = match self.middle.kind() {
            rustc_middle::ty::TyKind::Ref(_, ty, mutbl) => (ty, mutbl),
            _ => return None,
        };

        Some((
            Self {
                hir: hir_data,
                middle: *middle_data.0,
            },
            *middle_data.1,
        ))
    }

    pub(crate) fn span(&self) -> Span {
        match self.hir {
            Either::Left(hir) => hir.span,
            Either::Right(span) => span,
        }
    }

    pub(crate) fn get_generics_of_query(
        &self,
        ctx: &LateContext<'tcx>,
    ) -> Option<(Self, Option<Self>)> {
        if clippy_utils::ty::match_type(ctx, self.middle, bevy_paths::QUERY) {
            let hir = match self.hir {
                either::Either::Left(ty) => match get_generics_of_query_hir(ty) {
                    Some(generics) => Either::Left(generics),
                    None => return None,
                },
                either::Either::Right(span) => Either::Right(span),
            };

            if let Some(middle) = get_generics_of_query_middle(self.middle) {
                let world = Self {
                    hir: match hir {
                        Either::Left((hir_ty, _)) => Either::Left(hir_ty),
                        Either::Right(span) => Either::Right(span),
                    },
                    middle: middle.0,
                };

                let filter = match hir {
                    Either::Left(hir_ty) => hir_ty.1.and_then(|hir| {
                        middle.1.map(|middle| Self {
                            hir: Either::Left(hir),
                            middle,
                        })
                    }),
                    Either::Right(span) => middle.1.map(|middle| Self {
                        hir: Either::Right(span),
                        middle,
                    }),
                };

                return Some((world, filter));
            }
        }
        None
    }
}

fn get_generics_of_query_hir<'tcx>(
    query: &'tcx rustc_hir::Ty,
) -> Option<(&'tcx rustc_hir::Ty<'tcx>, Option<&'tcx rustc_hir::Ty<'tcx>>)> {
    if let rustc_hir::TyKind::Path(rustc_hir::QPath::Resolved(_, path)) = query.kind {
        if let Some(segment) = path.segments.iter().last() {
            if let Some(generic_args) = segment.args {
                if let Some(rustc_hir::GenericArg::Type(world)) = &generic_args.args.get(2) {
                    if let Some(rustc_hir::GenericArg::Type(filter)) = &generic_args.args.get(3) {
                        return Some((world, Some(filter)));
                    }

                    return Some((world, None));
                }
            }
        }
    }

    None
}

fn get_generics_of_query_middle(
    query: rustc_middle::ty::Ty,
) -> Option<(rustc_middle::ty::Ty, Option<rustc_middle::ty::Ty>)> {
    if let rustc_middle::ty::TyKind::Adt(_, generics) = query.kind() {
        if let Some(world) = generics.get(2).map(|generic| generic.expect_ty()) {
            let filter = generics.get(3).map(|generic| generic.expect_ty());
            return Some((world, filter));
        }
    }

    None
}
