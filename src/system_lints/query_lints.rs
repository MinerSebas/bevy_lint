use clippy_utils::diagnostics;
use itertools::Itertools;
use rustc_ast::Mutability;
use rustc_lint::LateContext;
use rustc_middle::ty::TyKind;
use rustc_session::declare_lint;
use rustc_span::Span;
use std::collections::{hash_map::Entry, HashMap};

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
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A, With<A>>) {}
    ///
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A>) {}
    ///
    /// # system.system();
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
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<Option<&A>, With<A>>) {}
    ///
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A>) {}
    ///
    /// # system.system();
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
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<Entity, Or<(With<A>,)>>) {}
    ///
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<Entity, With<A>>) {}
    ///
    /// # system.system();
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
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A, Without<A>>) {}
    ///
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// #
    /// fn system(query: Query<&A>) {}
    ///
    /// # system.system();
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
    /// # use bevy::ecs::prelude::*;
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
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
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
    /// # system.system();
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
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, Or<(Added<B>, Changed<B>)>) {}
    ///
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, Changed<B>>) {}
    ///
    /// # system.system();
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
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, (Added<B>, Changed<B>)>) {}
    ///
    /// # system.system();
    /// ```
    /// Instead do:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// #
    /// # #[derive(Component)]
    /// # struct A;
    /// # #[derive(Component)]
    /// # struct B;
    /// #
    /// fn system(mut query: Query<&A, Added<B>>) {}
    ///
    /// # system.system();
    /// ```
    pub UNNECESSARY_CHANGED,
    Warn,
    "Detects unnecessary `Changed` filters in Bevy query parameters."
}

#[allow(clippy::type_complexity)]
#[derive(Debug, Default, Clone)]
struct QueryData<'tcx> {
    data: HashMap<TyKind<'tcx>, Vec<(Mutability, Span)>>,
    option: Vec<(QueryData<'tcx>, Span)>,
    with: HashMap<TyKind<'tcx>, Vec<Span>>,
    without: HashMap<TyKind<'tcx>, Vec<Span>>,
    added: HashMap<TyKind<'tcx>, Vec<Span>>,
    changed: HashMap<TyKind<'tcx>, Vec<Span>>,
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
                if let Entry::Vacant(e) = self.data.entry(ty_kind.clone()) {
                    e.insert(vec![(*mutbl, *span)]);
                } else {
                    self.data.get_mut(ty_kind).unwrap().push((*mutbl, *span));
                }
            }
            WorldQuery::Option(world_query, span) => {
                let mut world = QueryData {
                    meta: QueryDataMeta::Option,
                    ..Default::default()
                };
                world.fill_with_world_query(ctx, &*world_query.0);
                self.option.push((world, *span));
            }
            WorldQuery::Filter(filter_query, span) => {
                diagnostics::span_lint(
                    ctx,
                    FILTER_IN_WORLD_QUERY,
                    *span,
                    "Usage of Filter in first Part of Query.",
                );

                self.fill_with_filter_query(filter_query)
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
                if !filter_querys.is_empty() {
                    let mut vec = Vec::new();
                    for filter_query in filter_querys {
                        match filter_query {
                            FilterQuery::Or(filter_querys, _) => {
                                // Todo: Lint here for nested or?
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
                } else {
                    self.or.push((Vec::new(), *span));
                }
            }
            FilterQuery::With(ty_kind, span) => {
                if let Entry::Vacant(e) = self.with.entry(ty_kind.clone()) {
                    e.insert(vec![*span]);
                } else {
                    self.with.get_mut(ty_kind).unwrap().push(*span);
                }
            }
            FilterQuery::Without(ty_kind, span) => {
                if let Entry::Vacant(e) = self.without.entry(ty_kind.clone()) {
                    e.insert(vec![*span]);
                } else {
                    self.with.get_mut(ty_kind).unwrap().push(*span);
                }
            }
            FilterQuery::Added(ty_kind, span) => {
                if let Entry::Vacant(e) = self.added.entry(ty_kind.clone()) {
                    e.insert(vec![*span]);
                } else {
                    self.with.get_mut(ty_kind).unwrap().push(*span);
                }
            }
            FilterQuery::Changed(ty_kind, span) => {
                if let Entry::Vacant(e) = self.changed.entry(ty_kind.clone()) {
                    e.insert(vec![*span]);
                } else {
                    self.with.get_mut(ty_kind).unwrap().push(*span);
                }
            }
        }
    }

    fn lint_query_data(&self, ctx: &LateContext, span: &Span, facts: &[&QueryData]) {
        let mut new_facts = Vec::from(facts);
        new_facts.push(self);
        self.check_for_unnecessary_or(ctx, &new_facts);

        for (def_id, data) in self.with.iter().sorted() {
            if self.data.contains_key(def_id)
                || self.added.contains_key(def_id)
                || self.changed.contains_key(def_id)
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

        for (def_id, data) in self.changed.iter().sorted() {
            if self.added.contains_key(def_id) {
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

        if let QueryDataMeta::Default = self.meta {
            let mut contradiction = false;

            for (def_id, data) in self.without.iter().sorted() {
                if self.data.contains_key(def_id)
                    || self.added.contains_key(def_id)
                    || self.changed.contains_key(def_id)
                    || self.with.contains_key(def_id)
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
        option: &(QueryData, Span),
        facts: &[&QueryData],
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
            vec_with.extend(fact.data.keys().chain(fact.with.keys()));
            vec_without.extend(fact.without.keys());
            vec_change.extend(fact.added.keys().chain(fact.changed.keys()));
        }

        let option_fact: Vec<_> = option.0.data.keys().chain(option.0.with.keys()).collect();

        let option_change: Vec<_> = option
            .0
            .added
            .keys()
            .chain(option.0.changed.keys())
            .collect();

        if (!option_fact
            .iter()
            .filter(|ty_kind| !vec_with.contains(ty_kind) && !vec_change.contains(ty_kind))
            .any(|_| true)
            && !option_change
                .iter()
                .filter(|ty_kind| !vec_change.contains(ty_kind))
                .any(|_| true)
            && !option
                .0
                .without
                .keys()
                .filter(|ty_kind| vec_with.contains(ty_kind))
                .any(|_| true))
            || (option.0.count() - option.0.option.len() == 0)
        {
            diagnostics::span_lint(
                ctx,
                UNNECESSARY_OPTION,
                option.1,
                "`Option` Query is always `Some`",
            );
        } else if option_fact
            .iter()
            .filter(|ty_kind| {
                vec_without.contains(*ty_kind) || option.0.without.keys().contains(*ty_kind)
            })
            .any(|_| true)
            || option
                .0
                .without
                .keys()
                .filter(|ty_kind| vec_with.contains(ty_kind))
                .any(|_| true)
        {
            diagnostics::span_lint(
                ctx,
                UNNECESSARY_OPTION,
                option.1,
                "`Option` Query is always `None`",
            );
        }
    }

    fn check_for_unnecessary_or(&self, ctx: &LateContext, facts: &[&QueryData]) {
        // Todo: Also handle nested ANDs `Or<(With<A>, Without<A>), With<B>)>`
        for concrete_or in &self.or {
            //dbg!(concrete_or);
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
                vec_with.extend(fact.data.keys().chain(fact.with.keys()));
                vec_without.extend(fact.without.keys());
                vec_change.extend(fact.added.keys().chain(fact.changed.keys()));
            }

            //dbg!(&vec_with);
            //dbg!(&concrete_or);

            for part in &concrete_or.0 {
                //dbg!(part);
                if !part
                    .with
                    .keys()
                    .filter(|ty_kind| !vec_with.contains(ty_kind) && !vec_change.contains(ty_kind))
                    .any(|_| true)
                    && !part
                        .added
                        .keys()
                        .chain(part.changed.keys())
                        .filter(|ty_kind| !vec_change.contains(ty_kind))
                        .any(|_| true)
                    && !part
                        .without
                        .keys()
                        .filter(|ty_kind| !vec_without.contains(ty_kind))
                        .any(|_| true)
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
                        if changed.contains_key(ty_kind) {
                            for span in spans {
                                diagnostics::span_lint(
                                    ctx,
                                    UNNECESSARY_ADDED,
                                    *span,
                                    "Unnecessary `Added` Filter",
                                )
                            }
                        }
                    }
                }
            }

            /*

             dbg!(&part.added);
             dbg!(&part.changed);
            for added in &part.added {
                 if part.changed.contains_key(&added.0) {
                     for span in added.1 {
                         diagnostics::span_lint(
                             ctx,
                             UNNECESSARY_ADDED,
                             *span,
                             "Unnecessary `Added` Filter",
                         )
                     }
                 }
             }*/
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

#[derive(Debug, Clone, Copy)]
enum QueryDataMeta {
    Default,
    Option,
}

impl Default for QueryDataMeta {
    fn default() -> Self {
        QueryDataMeta::Default
    }
}

pub(super) fn lint_query(ctx: &LateContext, query: Query) {
    //dbg!(&query);
    let query_data = {
        let mut query_data = QueryData::default();
        query_data.fill_with_world_query(ctx, &query.world_query);
        query_data.fill_with_filter_query(&query.filter_query);
        query_data
    };
    //dbg!(&query_data);

    query_data.lint_query_data(ctx, &query.span, &[]);
}
