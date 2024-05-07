use clippy_utils::diagnostics;
use itertools::Itertools;
use rustc_ast::Mutability;
use rustc_lint::LateContext;
use rustc_middle::ty::TyKind;
use rustc_session::declare_lint;
use rustc_span::Span;

use super::model::{FilterQuery, Query, WorldQuery};

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `With` query filters in Bevy query parameters.
    ///
    /// **Why is this bad?**
    /// The Filter does not effect the Results of a query, but still wasted space.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A, With<A>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    pub UNNECESSARY_WITH,
    Warn,
    "Detects unnecessary `With` query filters in Bevy query parameters."
}

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `Option` queries in Bevy query parameters.
    ///
    /// **Why is this bad?**
    /// The query will always return the `Some` Variant.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<Option<&A>, With<A>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    pub UNNECESSARY_OPTION,
    Warn,
    "Detects unnecessary `Option` queries in Bevy query parameters."
}

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `Or` query filters in Bevy query parameters.
    ///
    /// **Why is this bad?**
    /// The `Or` filters can be trivialy removed, without changing the Result of the query.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<Entity, Or<(With<A>,)>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<Entity, With<A>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    pub UNNECESSARY_OR,
    Warn,
    "Detects unnecessary `Or` filters in Bevy query parameters."
}

declare_lint! {
    /// **What it does:**
    /// Detects empty Queries that will never return any Data.
    /// Also triggered when a `Option` is used, that will always return `None`.
    ///
    /// **Why is this bad?**
    /// These Queries will never return any Data, because no Entity will fullfill the Requirements.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A, Without<A>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    pub EMPTY_QUERY,
    Warn,
    "Detects empty Queries."
}

declare_lint! {
    /// **What it does:**
    /// Detects Filters in the Data part of a Query.
    ///
    /// **Why is this bad?**
    ///
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<(&A, With<B>)>) {
    ///     for (component, filter) in query.iter_mut() {}
    /// }
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(query: Query<&A, With<B>>) {
    ///     for component in query.iter() {}
    /// }
    ///
    /// # assert_is_system(system);
    /// ```
    pub FILTER_IN_WORLD_QUERY,
    Warn,
    "Detects Filters in the Data part of a Query."
}

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `Added` filters in Bevy query parameters."
    ///
    /// **Why is this bad?**
    /// The `Changed` Filter also triggers for Component Additions.
    /// Thus combining them inside an `Or` makes `Added` unnecessary.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, Or<(Added<B>, Changed<B>)>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, Changed<B>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    pub UNNECESSARY_ADDED,
    Warn,
    "Detects unnecessary `Added` filters in Bevy query parameters."
}

declare_lint! {
    /// **What it does:**
    /// Detects unnecessary `Changed` filters in Bevy query parameters."
    ///
    /// **Why is this bad?**
    /// The `Changed` Filter also triggers for Component Additions.
    /// Thus combining them inside an `Or` makes `Added` unnecessary.
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, (Added<B>, Changed<B>)>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::{prelude::*, system::{assert_is_system, SystemParam}};
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, Added<B>>) {}
    ///
    /// # assert_is_system(system);
    /// ```
    pub UNNECESSARY_CHANGED,
    Warn,
    "Detects unnecessary `Changed` filters in Bevy query parameters."
}

#[allow(clippy::type_complexity)]
#[derive(Debug, Default, Clone)]
struct QueryData<'tcx> {
    data: Vec<(TyKind<'tcx>, Vec<(Mutability, Span)>)>,
    option: Vec<(QueryData<'tcx>, Span)>,
    with: Vec<(TyKind<'tcx>, Vec<Span>)>,
    without: Vec<(TyKind<'tcx>, Vec<Span>)>,
    added: Vec<(TyKind<'tcx>, Vec<Span>)>,
    changed: Vec<(TyKind<'tcx>, Vec<Span>)>,
    or: Vec<(Vec<QueryData<'tcx>>, Span)>,
    meta: QueryDataMeta,
}

impl<'tcx> QueryData<'tcx> {
    fn fill_with_world_query(&mut self, ctx: &LateContext, world_query: &WorldQuery<'tcx>) {
        match world_query {
            WorldQuery::Tuple(world_querys, _) => {
                for world_query in world_querys {
                    self.fill_with_world_query(ctx, world_query);
                }
            }
            WorldQuery::Data(ty_kind, mutbl, span) => {
                for (kind, vec) in &mut self.data {
                    if ty_kind == kind {
                        vec.push((*mutbl, *span));
                        return;
                    }
                }

                self.data.push((*ty_kind, vec![(*mutbl, *span)]));
            }
            WorldQuery::Option(world_query, span) => {
                let mut world = QueryData {
                    meta: QueryDataMeta::Option,
                    ..Default::default()
                };
                world.fill_with_world_query(ctx, &world_query.0);
                self.option.push((world, *span));
            }
            WorldQuery::Filter(filter_query, span) => {
                diagnostics::span_lint(
                    ctx,
                    FILTER_IN_WORLD_QUERY,
                    *span,
                    "Usage of Filter in first Part of Query.",
                );

                self.fill_with_filter_query(filter_query);
            }
        }
    }

    fn fill_with_filter_query(&mut self, filter_query: &FilterQuery<'tcx>) {
        match filter_query {
            FilterQuery::Tuple(filter_querys, _) => {
                for filter_query in filter_querys {
                    self.fill_with_filter_query(filter_query);
                }
            }
            FilterQuery::Or(filter_querys, span) => {
                let mut vec = Vec::new();
                for filter_query in filter_querys {
                    match filter_query {
                        FilterQuery::Or(filter_querys, _) => {
                            // TODO: Lint here for nested or?
                            for filter_query in filter_querys {
                                let mut data = QueryData::default();
                                data.fill_with_filter_query(filter_query);
                                vec.push(data);
                            }
                        }
                        FilterQuery::Tuple(_, _)
                        | FilterQuery::With(_, _)
                        | FilterQuery::Without(_, _)
                        | FilterQuery::Added(_, _)
                        | FilterQuery::Changed(_, _) => {
                            let mut data = QueryData::default();
                            data.fill_with_filter_query(filter_query);
                            vec.push(data);
                        }
                    }
                }
                self.or.push((vec, *span));
            }
            FilterQuery::With(ty_kind, span) => {
                for (kind, vec) in &mut self.with {
                    if ty_kind == kind {
                        vec.push(*span);
                        return;
                    }
                }

                self.with.push((*ty_kind, vec![*span]));
            }
            FilterQuery::Without(ty_kind, span) => {
                for (kind, vec) in &mut self.without {
                    if ty_kind == kind {
                        vec.push(*span);
                        return;
                    }
                }

                self.without.push((*ty_kind, vec![*span]));
            }
            FilterQuery::Added(ty_kind, span) => {
                for (kind, vec) in &mut self.added {
                    if ty_kind == kind {
                        vec.push(*span);
                        return;
                    }
                }

                self.added.push((*ty_kind, vec![*span]));
            }
            FilterQuery::Changed(ty_kind, span) => {
                for (kind, vec) in &mut self.changed {
                    if ty_kind == kind {
                        vec.push(*span);
                        return;
                    }
                }

                self.changed.push((*ty_kind, vec![*span]));
            }
        }
    }

    fn lint_query_data(&self, ctx: &LateContext, span: &Span, facts: &[&QueryData<'tcx>]) {
        let mut new_facts = Vec::from(facts);
        new_facts.push(self);
        self.check_for_unnecessary_or(ctx, &new_facts);

        for (ty_kind, data) in self.with.iter() {
            if contains_ty_kind(&self.data, ty_kind)
                || contains_ty_kind(&self.added, ty_kind)
                || contains_ty_kind(&self.changed, ty_kind)
            {
                for inst in data {
                    diagnostics::span_lint(
                        ctx,
                        UNNECESSARY_WITH,
                        *inst,
                        "Unnecessary `With` Filter",
                    );
                }
            }
        }

        for (ty_kind, data) in self.changed.iter() {
            if contains_ty_kind(&self.added, ty_kind) {
                for inst in data {
                    diagnostics::span_lint(
                        ctx,
                        UNNECESSARY_CHANGED,
                        *inst,
                        "Unnecessary `Changed` Filter",
                    );
                }
            }
        }

        if QueryDataMeta::Default == self.meta {
            let mut contradiction = false;

            for (ty_kind, data) in self.without.iter() {
                if contains_ty_kind(&self.data, ty_kind)
                    || contains_ty_kind(&self.added, ty_kind)
                    || contains_ty_kind(&self.changed, ty_kind)
                    || contains_ty_kind(&self.with, ty_kind)
                {
                    contradiction = true;

                    for _ in data {
                        diagnostics::span_lint(ctx, EMPTY_QUERY, *span, "Empty Query");
                    }
                }
            }

            if !contradiction {
                for option in &self.option {
                    QueryData::check_for_unnecessary_option(ctx, option, &[self]);
                }
            }
        }
    }

    fn check_for_unnecessary_option(
        ctx: &LateContext,
        option: &(QueryData<'tcx>, Span),
        facts: &[&QueryData<'tcx>],
    ) {
        if option.0.count() >= 2 {
            option.0.lint_query_data(ctx, &option.1, facts);
        }

        if !option.0.option.is_empty() {
            let mut new_facts = Vec::from(facts);
            new_facts.push(&option.0);

            for sub_option in &option.0.option {
                QueryData::check_for_unnecessary_option(ctx, sub_option, &new_facts);
            }
        }

        let mut vec_with = Vec::new();
        let mut vec_without = Vec::new();
        let mut vec_change = Vec::new();

        for fact in facts {
            vec_with.extend(keys(&fact.data).chain(keys(&fact.with)));
            vec_without.extend(keys(&fact.without));
            vec_change.extend(keys(&fact.added).chain(keys(&fact.changed)));
        }

        let option_fact: Vec<_> = keys(&option.0.data).chain(keys(&option.0.with)).collect();

        let option_change: Vec<_> = keys(&option.0.added)
            .chain(keys(&option.0.changed))
            .collect();

        if (!option_fact
            .iter()
            .any(|ty_kind| !vec_with.contains(ty_kind) && !vec_change.contains(ty_kind))
            && !option_change
                .iter()
                .any(|ty_kind| !vec_change.contains(ty_kind))
            && !keys(&option.0.without).any(|ty_kind| vec_with.contains(&ty_kind)))
            || (option.0.count() - option.0.option.len() == 0)
        {
            diagnostics::span_lint(
                ctx,
                UNNECESSARY_OPTION,
                option.1,
                "`Option` Query is always `Some`",
            );
        } else if option_fact.iter().any(|ty_kind| {
            vec_without.contains(ty_kind) || keys(&option.0.without).contains(ty_kind)
        }) || keys(&option.0.without).any(|ty_kind| vec_with.contains(&ty_kind))
        {
            diagnostics::span_lint(
                ctx,
                UNNECESSARY_OPTION,
                option.1,
                "`Option` Query is always `None`",
            );
        }
    }

    fn check_for_unnecessary_or(&self, ctx: &LateContext, facts: &[&QueryData<'tcx>]) {
        // TODO: Also handle nested ANDs `Or<(With<A>, Without<A>), With<B>)>`
        for concrete_or in &self.or {
            if concrete_or.0.len() < 2 {
                diagnostics::span_lint(
                    ctx,
                    UNNECESSARY_OPTION,
                    concrete_or.1,
                    "Unnecessary `Or` Filter",
                );
                return;
            }

            let mut vec_with = Vec::new();
            let mut vec_without = Vec::new();
            let mut vec_change = Vec::new();

            for fact in facts {
                vec_with.extend(keys(&fact.data).chain(keys(&fact.with)));
                vec_without.extend(keys(&fact.without));
                vec_change.extend(keys(&fact.added).chain(keys(&fact.changed)));
            }

            for part in &concrete_or.0 {
                if !keys(&part.with)
                    .any(|ty_kind| !vec_with.contains(&ty_kind) && !vec_change.contains(&ty_kind))
                    && !keys(&part.added)
                        .chain(keys(&part.changed))
                        .any(|ty_kind| !vec_change.contains(&ty_kind))
                    && !keys(&part.without).any(|ty_kind| !vec_without.contains(&ty_kind))
                {
                    diagnostics::span_lint(
                        ctx,
                        UNNECESSARY_OPTION,
                        concrete_or.1,
                        "Unnecessary `Or` Filter",
                    );
                    break;
                }
            }

            for (added_index, added) in concrete_or
                .0
                .iter()
                .enumerate()
                .filter(|(_, data)| !data.added.is_empty())
                .map(|(index, data)| (index, &data.added))
            {
                for changed in concrete_or
                    .0
                    .iter()
                    .enumerate()
                    .filter(|(index, data)| *index != added_index && !data.changed.is_empty())
                    .map(|(_, data)| &data.changed)
                {
                    for (ty_kind, spans) in added {
                        if contains_ty_kind(changed, ty_kind) {
                            for span in spans {
                                diagnostics::span_lint(
                                    ctx,
                                    UNNECESSARY_ADDED,
                                    *span,
                                    "Unnecessary `Added` Filter",
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.data.len()
            + self.option.len()
            + self.with.len()
            + self.without.len()
            + self.added.len()
            + self.changed.len()
            + self.or.len()
    }
}

fn contains_ty_kind<'tcx, T>(vec: &Vec<(TyKind<'tcx>, T)>, ty_kind: &TyKind<'tcx>) -> bool {
    for (kind, _) in vec {
        if kind == ty_kind {
            return true;
        }
    }
    false
}

fn keys<'t, 'tcx, T>(vec: &'t Vec<(TyKind<'tcx>, T)>) -> impl Iterator<Item = TyKind<'tcx>> + 't {
    vec.into_iter().map(|(ty_kind, _)| *ty_kind)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QueryDataMeta {
    Default,
    Option,
}

impl Default for QueryDataMeta {
    fn default() -> Self {
        Self::Default
    }
}

pub(super) fn lint_query(ctx: &LateContext, query: Query) {
    let query_data = {
        let mut query_data = QueryData::default();
        query_data.fill_with_world_query(ctx, &query.world_query);
        query_data.fill_with_filter_query(&query.filter_query);
        query_data
    };

    query_data.lint_query_data(ctx, &query.span, &[]);
}
