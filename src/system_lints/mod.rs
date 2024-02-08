use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::Span;

mod model;
pub mod query_lints;

use self::{
    model::{FilterQuery, Query, SystemParamType, WorldQuery},
    query_lints::{
        lint_query, EMPTY_QUERY, FILTER_IN_WORLD_QUERY, UNNECESSARY_ADDED, UNNECESSARY_CHANGED,
        UNNECESSARY_OPTION, UNNECESSARY_OR, UNNECESSARY_WITH,
    },
};
use super::{bevy_paths, mixed_ty::MixedTy};

declare_lint_pass!(SystemLintPass =>
    [
        EMPTY_QUERY,
        FILTER_IN_WORLD_QUERY,
        UNNECESSARY_ADDED,
        UNNECESSARY_CHANGED,
        UNNECESSARY_OPTION,
        UNNECESSARY_OR,
        UNNECESSARY_WITH
    ]
);

impl<'tcx> LateLintPass<'tcx> for SystemLintPass {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html

    fn check_item(&mut self, ctx: &LateContext<'tcx>, item: &'tcx rustc_hir::Item<'tcx>) {
        if let Some(system_params) = match &item.kind {
            rustc_hir::ItemKind::Fn(_, _, _) => MixedTy::fn_inputs_from_fn_item(ctx, item),
            rustc_hir::ItemKind::Struct(rustc_hir::VariantData::Struct { .. }, _) => {
                let Some(system_param_def_id) =
                    clippy_utils::get_trait_def_id(ctx, bevy_paths::SYSTEM_PARAM)
                else {
                    return;
                };

                if !clippy_utils::ty::implements_trait(
                    ctx,
                    ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
                    system_param_def_id,
                    &[],
                ) {
                    return;
                }

                MixedTy::fields_from_struct_item(ctx, item)
            }
            _ => return,
        } {
            lint_function_signature(ctx, &system_params);
        }
    }

    fn check_trait_item(
        &mut self,
        ctx: &LateContext<'tcx>,
        item: &'tcx rustc_hir::TraitItem<'tcx>,
    ) {
        match item.kind {
            rustc_hir::TraitItemKind::Fn(_, _) => (),
            _ => return,
        }

        if let Some(system_params) = MixedTy::fn_inputs_from_trait_fn_item(ctx, item) {
            lint_function_signature(ctx, &system_params);
        }
    }

    fn check_impl_item(&mut self, ctx: &LateContext<'tcx>, item: &'tcx rustc_hir::ImplItem<'tcx>) {
        match item.kind {
            rustc_hir::ImplItemKind::Fn(_, _) => (),
            _ => return,
        }

        if let Some(system_params) = MixedTy::fn_inputs_from_impl_fn_item(ctx, item) {
            lint_function_signature(ctx, &system_params);
        }
    }
}

fn lint_function_signature<'tcx>(ctx: &LateContext<'tcx>, inputs: &[MixedTy<'tcx>]) {
    let system_params = inputs
        .iter()
        .filter_map(|mixed_ty| recursively_resolve_system_param(ctx, mixed_ty))
        .map(|mut system_param| {
            system_param.remove_substitutions();
            system_param
        });

    for system_param in system_params {
        recursively_lint_system_param(ctx, system_param);
    }
}

fn recursively_lint_system_param(ctx: &LateContext, system_param: SystemParamType) {
    match system_param {
        SystemParamType::Tuple(system_params, _) => {
            for system_param in system_params {
                recursively_lint_system_param(ctx, system_param);
            }
        }
        SystemParamType::Query(query, _) => {
            lint_query(ctx, query);
        }
    }
}

fn recursively_resolve_system_param<'tcx>(
    ctx: &LateContext<'tcx>,
    ty: &MixedTy<'tcx>,
) -> Option<SystemParamType<'tcx>> {
    if let Some(types) = ty.extract_tuple_types() {
        let vec = types
            .iter()
            .filter_map(|ty| recursively_resolve_system_param(ctx, ty))
            .collect();

        Some(SystemParamType::Tuple(vec, ty.span()))
    } else if clippy_utils::ty::match_type(ctx, ty.middle, bevy_paths::QUERY) {
        resolve_query(ctx, ty).map(|query| SystemParamType::Query(query, ty.span()))
    } else {
        None
    }
}

fn resolve_query<'tcx>(ctx: &LateContext<'tcx>, ty: &MixedTy<'tcx>) -> Option<Query<'tcx>> {
    ty.get_generics_of_query(ctx).map(|(world, filter)| {
        let resolved_world = recursively_resolve_world_query(ctx, &world)
            .unwrap_or_else(|| WorldQuery::Tuple(Vec::new(), ty.span()));
        let resolved_filter = filter
            .and_then(|filter| recursively_resolve_filter_query(ctx, &filter))
            .map_or_else(
                || FilterQuery::Tuple(Vec::new(), ty.span()),
                |resolved_filter| resolved_filter,
            );

        Query {
            world_query: resolved_world,
            filter_query: resolved_filter,
            span: ty.span(),
        }
    })
}

fn recursively_resolve_world_query<'tcx>(
    ctx: &LateContext<'tcx>,
    world: &MixedTy<'tcx>,
) -> Option<WorldQuery<'tcx>> {
    match world.middle.kind() {
        rustc_middle::ty::TyKind::Tuple(_) => world.extract_tuple_types().map(|types| {
            let vec = types
                .iter()
                .filter_map(|ty| recursively_resolve_world_query(ctx, ty))
                .collect();

            WorldQuery::Tuple(vec, world.span())
        }),
        rustc_middle::ty::TyKind::Adt(_, _) => {
            if clippy_utils::ty::match_type(ctx, world.middle, &["core", "option", "Option"]) {
                let generics = world.extract_generics_from_struct().unwrap();
                assert_eq!(generics.len(), 1);
                recursively_resolve_world_query(ctx, &generics[0]).map(|world_query| {
                    let span = *world_query.span();
                    WorldQuery::Option((Box::new(world_query), span), world.span())
                })
            } else if clippy_utils::ty::match_type(ctx, world.middle, bevy_paths::OR)
                || clippy_utils::ty::match_type(ctx, world.middle, bevy_paths::ADDED)
                || clippy_utils::ty::match_type(ctx, world.middle, bevy_paths::CHANGED)
                || clippy_utils::ty::match_type(ctx, world.middle, bevy_paths::WITH)
                || clippy_utils::ty::match_type(ctx, world.middle, bevy_paths::WITHOUT)
            {
                recursively_resolve_filter_query(ctx, world)
                    .map(|filter| WorldQuery::Filter(filter, world.span()))
            } else {
                None
            }
        }
        rustc_middle::ty::TyKind::Ref(_, _, _) => world
            .strip_reference()
            .map(|(ty, mutbl)| WorldQuery::Data(*ty.middle.kind(), mutbl, world.span())),
        _ => None,
    }
}

fn recursively_resolve_filter_query<'tcx>(
    ctx: &LateContext<'tcx>,
    filter: &MixedTy<'tcx>,
) -> Option<FilterQuery<'tcx>> {
    if let Some(types) = filter.extract_tuple_types() {
        let vec = types
            .iter()
            .filter_map(|ty| recursively_resolve_filter_query(ctx, ty))
            .collect();

        Some(FilterQuery::Tuple(vec, filter.span()))
    } else if clippy_utils::ty::match_type(ctx, filter.middle, bevy_paths::OR) {
        let generics = filter.extract_generics_from_struct().unwrap();
        assert_eq!(generics.len(), 1);

        let vec = generics[0]
            .extract_tuple_types()
            .unwrap()
            .iter()
            .filter_map(|ty| recursively_resolve_filter_query(ctx, ty))
            .collect();

        Some(FilterQuery::Or(vec, filter.span()))
    } else {
        let constructor: Option<fn(rustc_middle::ty::TyKind<'tcx>, Span) -> FilterQuery<'tcx>> = {
            if clippy_utils::ty::match_type(ctx, filter.middle, bevy_paths::ADDED) {
                Some(FilterQuery::Added)
            } else if clippy_utils::ty::match_type(ctx, filter.middle, bevy_paths::CHANGED) {
                Some(FilterQuery::Changed)
            } else if clippy_utils::ty::match_type(ctx, filter.middle, bevy_paths::WITH) {
                Some(FilterQuery::With)
            } else if clippy_utils::ty::match_type(ctx, filter.middle, bevy_paths::WITHOUT) {
                Some(FilterQuery::Without)
            } else {
                None
            }
        };

        constructor.map(|constructor| {
            let generics = filter.extract_generics_from_struct().unwrap();
            assert_eq!(generics.len(), 1);

            let filter_query = constructor(*generics[0].middle.kind(), filter.span());

            filter_query
        })
    }
}
