use clippy_utils::diagnostics::span_lint;
use rustc_hir::{
    hir_id::HirId, intravisit::FnKind, Body, FnDecl, GenericArg, Path, QPath, Ty, TyKind,
    VariantData,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::{def_id::DefId, Span};

use crate::{bevy_helpers, bevy_paths};

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `With` query filters in Bevy query parameters.
    /// **Why is this bad?**
    /// The Filter does not effect the Results of a query, but still wasted space.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// # use bevy_ecs::query::With;
    /// fn system(query: Query<&A, With<A>>) {}
    /// ```
    /// Use instead:
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// fn system(query: Query<&A>) {}
    /// ```
    pub UNNECESSARY_WITH,
    Warn,
    "Detects unnecessary `With` query filters in Bevy query parameters."
}

declare_lint_pass!(UnnecessaryWith => [UNNECESSARY_WITH]);

impl<'hir> LateLintPass<'hir> for UnnecessaryWith {
    // A list of things you might check can be found here:
    // https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/trait.LateLintPass.html

    fn check_fn(
        &mut self,
        ctx: &LateContext<'hir>,
        _: FnKind<'hir>,
        decl: &'hir FnDecl<'hir>,
        _: &'hir Body<'hir>,
        _: Span,
        _: HirId,
    ) {
        for typ in decl.inputs {
            recursively_search_type(
                ctx,
                typ,
                bevy_paths::QUERY,
                check_for_unnecesarry_query_parameters,
            );
        }
    }

    fn check_struct_def(&mut self, ctx: &LateContext<'hir>, data: &'hir VariantData<'hir>) {
        // Todo: Filter out Types that dont implement SystemParam
        // -> Is it possible to go from rustc_hir::Ty to rustc_middle::Ty?
        // Required for using clippy_utils::ty::implements_trait.
        if let VariantData::Struct(fields, _) = data {
            for field in *fields {
                recursively_search_type(
                    ctx,
                    field.ty,
                    bevy_paths::QUERY,
                    check_for_unnecesarry_query_parameters,
                )
            }
        }
    }
}

fn recursively_search_type<'hir, T: Fn(&LateContext<'hir>, &'hir Ty<'hir>) + Copy>(
    ctx: &LateContext<'hir>,
    typ: &'hir Ty,
    symbol_path: &[&str],
    function: T,
) {
    match typ.kind {
        TyKind::Path(QPath::Resolved(_, path)) => {
            if bevy_helpers::path_matches_symbol_path(ctx, path, symbol_path) {
                function(ctx, &typ)
            }
        }
        TyKind::Tup(types) => {
            for tup_typ in types {
                // Todo: Filter out Types that dont implement SystemParam
                // -> Is it possible to go from rustc_hir::Ty to rustc_middle::Ty?
                // Required for using clippy_utils::ty::implements_trait.
                recursively_search_type(ctx, tup_typ, symbol_path, function);
            }
        }
        _ => (),
    }
}

fn check_for_unnecesarry_query_parameters<'hir>(ctx: &LateContext<'hir>, query: &'hir Ty<'hir>) {
    if let Some((world, Some(filter))) = bevy_helpers::get_generics_of_query(ctx, query) {
        let mut parameters = QueryParameters::default();

        match &world.kind {
            TyKind::Rptr(_, mut_type) => {
                if let Some(def_id) = bevy_helpers::get_def_id_of_referenced_type(&mut_type) {
                    parameters
                        .data_querys
                        .push(QueryParameter::new(def_id, world.span));
                }
            }
            TyKind::Tup(types) => {
                for typ in *types {
                    if let TyKind::Rptr(_, mut_type) = &typ.kind {
                        if let Some(def_id) = bevy_helpers::get_def_id_of_referenced_type(&mut_type)
                        {
                            parameters
                                .data_querys
                                .push(QueryParameter::new(def_id, typ.span));
                        }
                    }
                }
            }
            _ => (),
        }

        match &filter.kind {
            TyKind::Path(QPath::Resolved(_, path)) => {
                if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::OR) {
                    parameters.with_querys.extend(check_or_filter(ctx, path));
                }
                if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::WITH) {
                    if let Some(def_id) = bevy_helpers::get_def_id_of_first_generic_arg(path) {
                        parameters
                            .with_querys
                            .push(QueryParameter::new(def_id, filter.span));
                    }
                }
            }
            TyKind::Tup(types) => {
                for typ in *types {
                    if let TyKind::Path(QPath::Resolved(_, path)) = typ.kind {
                        if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::OR) {
                            parameters.with_querys.extend(check_or_filter(ctx, path));
                        }
                        if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::ADDED)
                            || bevy_helpers::path_matches_symbol_path(
                                ctx,
                                path,
                                bevy_paths::CHANGED,
                            )
                        {
                            if let Some(def_id) =
                                bevy_helpers::get_def_id_of_first_generic_arg(path)
                            {
                                parameters
                                    .change_detection_querys
                                    .push(QueryParameter::new(def_id, typ.span));
                            }
                        }
                        if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::WITH) {
                            if let Some(def_id) =
                                bevy_helpers::get_def_id_of_first_generic_arg(path)
                            {
                                parameters
                                    .with_querys
                                    .push(QueryParameter::new(def_id, typ.span));
                            }
                        }
                    }
                }
            }
            _ => (),
        }

        parameters.check_for_unnecesarry_with(ctx);
    }
}

fn check_or_filter<'hir>(ctx: &LateContext<'hir>, path: &Path) -> Vec<QueryParameter> {
    let mut parameters = QueryParameters::default();

    if let Some(segment) = path.segments.iter().last() {
        if let Some(generic_args) = segment.args {
            if let GenericArg::Type(tuple) = &generic_args.args[0] {
                if let TyKind::Tup(types) = tuple.kind {
                    for typ in types {
                        if let TyKind::Path(QPath::Resolved(_, path)) = typ.kind {
                            if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::ADDED)
                                || bevy_helpers::path_matches_symbol_path(
                                    ctx,
                                    path,
                                    bevy_paths::CHANGED,
                                )
                            {
                                if let Some(def_id) =
                                    bevy_helpers::get_def_id_of_first_generic_arg(path)
                                {
                                    parameters
                                        .change_detection_querys
                                        .push(QueryParameter::new(def_id, typ.span));
                                }
                            }
                            if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::WITH) {
                                if let Some(def_id) =
                                    bevy_helpers::get_def_id_of_first_generic_arg(path)
                                {
                                    parameters
                                        .with_querys
                                        .push(QueryParameter::new(def_id, typ.span));
                                }
                            }
                        }
                    }

                    parameters.check_for_unnecesarry_with(ctx);
                }
            }
        }
    }

    parameters.with_querys
}

#[derive(Debug)]
struct QueryParameter {
    def_id: DefId,
    span: Span,
    lint_triggered: bool,
}

impl QueryParameter {
    fn new(def_id: DefId, span: Span) -> Self {
        Self {
            def_id,
            span,
            lint_triggered: false,
        }
    }
}

#[derive(Debug, Default)]
struct QueryParameters {
    data_querys: Vec<QueryParameter>,
    optional_querys: Vec<QueryParameter>,
    with_querys: Vec<QueryParameter>,
    without_querys: Vec<QueryParameter>,
    with_bundle_querys: Vec<QueryParameter>,
    change_detection_querys: Vec<QueryParameter>,
}

impl QueryParameters {
    fn check_for_unnecesarry_with<'hir>(&mut self, ctx: &LateContext<'hir>) {
        let iterator = self.data_querys.iter().chain(&self.change_detection_querys);

        for mut with_query in self
            .with_querys
            .iter_mut()
            .filter(|parameter| !parameter.lint_triggered)
        {
            if iterator
                .clone()
                .any(|parameter| parameter.def_id == with_query.def_id)
            {
                span_lint(
                    ctx,
                    UNNECESSARY_WITH,
                    with_query.span,
                    "Unnecessary `With` Filter",
                );
                with_query.lint_triggered = true;
            }
        }
    }
}
