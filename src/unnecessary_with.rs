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

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `Option` queries in Bevy query parameters.
    /// **Why is this bad?**
    /// The query will always return the `Some` Variant.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// # use bevy_ecs::query::With;
    /// fn system(query: Query<Option<&A>, With<A>>) {}
    /// ```
    /// Use instead:
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// fn system(query: Query<&A>) {}
    /// ```
    pub UNNECESSARY_OPTION,
    Warn,
    "Detects unnecessary `Option` queries in Bevy query parameters."
}

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `Or` filters in Bevy query parameters.
    /// **Why is this bad?**
    /// The `Or` filters can be trivialy removed, without changing the Result of the query.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// # use bevy_ecs::query::With;
    /// fn system(query: Query<Option<&A>, With<A>>) {}
    /// ```
    /// Use instead:
    /// ```rust
    /// # use bevy_ecs::system::Query;
    /// fn system(query: Query<&A>) {}
    /// ```
    pub UNNECESSARY_OR,
    Warn,
    "Detects unnecessary `Or` filters in Bevy query parameters."
}

declare_lint_pass!(QueryParametersLintPass => [UNNECESSARY_WITH, UNNECESSARY_OPTION, UNNECESSARY_OR]);

impl<'hir> LateLintPass<'hir> for QueryParametersLintPass {
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
            TyKind::Path(QPath::Resolved(_, path)) => {
                if bevy_helpers::path_matches_symbol_path(ctx, path, &clippy_utils::paths::OPTION) {
                    if let Some(def_id) = bevy_helpers::get_def_id_of_first_generic_arg(path) {
                        parameters
                            .data_parameters
                            .optional_querys
                            .push(QueryParameter::new(def_id, world.span))
                    }
                }
            }
            TyKind::Rptr(_, mut_type) => {
                if let Some(def_id) = bevy_helpers::get_def_id_of_referenced_type(&mut_type) {
                    parameters
                        .data_parameters
                        .data_querys
                        .push(QueryParameter::new(def_id, world.span));
                }
            }
            TyKind::Tup(types) => {
                for typ in *types {
                    match &typ.kind {
                        TyKind::Path(QPath::Resolved(_, path)) => {
                            if bevy_helpers::path_matches_symbol_path(
                                ctx,
                                path,
                                &clippy_utils::paths::OPTION,
                            ) {
                                parameters.data_parameters.optional_querys.push(
                                    QueryParameter::new(
                                        bevy_helpers::get_def_id_of_first_generic_arg(path)
                                            .unwrap(),
                                        typ.span,
                                    ),
                                )
                            }
                        }
                        TyKind::Rptr(_, mut_type) => {
                            if let Some(def_id) =
                                bevy_helpers::get_def_id_of_referenced_type(&mut_type)
                            {
                                parameters
                                    .data_parameters
                                    .data_querys
                                    .push(QueryParameter::new(def_id, typ.span));
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }

        match &filter.kind {
            TyKind::Path(QPath::Resolved(_, path)) => {
                if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::OR) {
                    parameters.extend_or_filter_parameters(&check_or_filter(ctx, path));
                }
                if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::ADDED)
                    || bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::CHANGED)
                {
                    if let Some(def_id) = bevy_helpers::get_def_id_of_first_generic_arg(path) {
                        parameters
                            .filter_parameters
                            .change_detection_querys
                            .push(QueryParameter::new(def_id, filter.span));
                    }
                }
                if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::WITH) {
                    if let Some(def_id) = bevy_helpers::get_def_id_of_first_generic_arg(path) {
                        parameters
                            .filter_parameters
                            .with_querys
                            .push(QueryParameter::new(def_id, filter.span));
                    }
                }
            }
            TyKind::Tup(types) => {
                for typ in *types {
                    if let TyKind::Path(QPath::Resolved(_, path)) = typ.kind {
                        if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::OR) {
                            parameters.extend_or_filter_parameters(&check_or_filter(ctx, path));
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
                                    .filter_parameters
                                    .change_detection_querys
                                    .push(QueryParameter::new(def_id, typ.span));
                            }
                        }
                        if bevy_helpers::path_matches_symbol_path(ctx, path, bevy_paths::WITH) {
                            if let Some(def_id) =
                                bevy_helpers::get_def_id_of_first_generic_arg(path)
                            {
                                parameters
                                    .filter_parameters
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
        parameters.check_for_unnecesarry_option(ctx);
    }
}

fn check_or_filter<'hir>(ctx: &LateContext<'hir>, path: &Path) -> FilterParameter {
    let mut parameters = FilterParameter::default();

    if let Some(segment) = path.segments.iter().last() {
        if let Some(generic_args) = segment.args {
            if let GenericArg::Type(tuple) = &generic_args.args[0] {
                if let TyKind::Tup(types) = tuple.kind {
                    if types.len() < 2 {
                        span_lint(
                            ctx,
                            UNNECESSARY_OPTION,
                            path.span,
                            "Unnecessary `Or` Filter",
                        );
                        return parameters;
                    }

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

                    QueryParameters {
                        filter_parameters: parameters.clone(),
                        ..Default::default()
                    }
                    .check_for_unnecesarry_with(ctx);
                }
            }
        }
    }

    parameters
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Default, Clone)]
struct DataParameter {
    data_querys: Vec<QueryParameter>,
    optional_querys: Vec<QueryParameter>,
}
#[derive(Debug, Default, Clone)]
struct FilterParameter {
    with_querys: Vec<QueryParameter>,
    without_querys: Vec<QueryParameter>,
    with_bundle_querys: Vec<QueryParameter>,
    change_detection_querys: Vec<QueryParameter>,
}

#[derive(Debug, Default, Clone)]
struct QueryParameters {
    data_parameters: DataParameter,
    filter_parameters: FilterParameter,
    or_filter_parameters: FilterParameter,
}

impl QueryParameters {
    fn extend_or_filter_parameters(&mut self, or_parameters: &FilterParameter) {
        self.or_filter_parameters
            .with_querys
            .extend(or_parameters.with_querys.clone());
        self.or_filter_parameters
            .without_querys
            .extend(or_parameters.without_querys.clone());
        self.or_filter_parameters
            .with_bundle_querys
            .extend(or_parameters.with_bundle_querys.clone());
        self.or_filter_parameters
            .change_detection_querys
            .extend(or_parameters.change_detection_querys.clone());
    }

    fn check_for_unnecesarry_with<'hir>(&mut self, ctx: &LateContext<'hir>) {
        let iterator = self
            .data_parameters
            .data_querys
            .iter()
            .chain(&self.filter_parameters.change_detection_querys);

        for mut with_query in self
            .filter_parameters
            .with_querys
            .iter_mut()
            .chain(&mut self.or_filter_parameters.with_querys)
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

    fn check_for_unnecesarry_option<'hir>(&mut self, ctx: &LateContext<'hir>) {
        let iterator = self
            .data_parameters
            .data_querys
            .iter()
            .chain(&self.filter_parameters.change_detection_querys)
            .chain(&self.filter_parameters.with_querys);

        for mut optional_query in self
            .data_parameters
            .optional_querys
            .iter_mut()
            .filter(|parameter| !parameter.lint_triggered)
        {
            if iterator
                .clone()
                .any(|parameter| parameter.def_id == optional_query.def_id)
            {
                span_lint(
                    ctx,
                    UNNECESSARY_OPTION,
                    optional_query.span,
                    "Unnecessary `Option` Query",
                );
                optional_query.lint_triggered = true;
            }
        }
    }
}
