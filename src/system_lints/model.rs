use rustc_span::{Span, Symbol};

#[derive(Debug, Clone)]
pub enum SystemParamType<'tcx> {
    //Alias(Vec<SystemParamType>, Span),
    //Derive(Vec<SystemParamType>, Span),
    Tuple(Vec<SystemParamType<'tcx>>),
    Query(Query<'tcx>),
    //QuerySet(Vec<(Query, Span)>, Span),
    //Resource(Resource),
    //Option(Resource),
    //Commands,
    //Local,
    //EventReader,
    //EventWriter,
}

impl<'tcx> SystemParamType<'tcx> {
    pub fn remove_substitutions(&mut self) {
        match self {
            SystemParamType::Tuple(system_params) => {
                for system_param in system_params {
                    system_param.remove_substitutions();
                }
            }
            SystemParamType::Query(query) => query.remove_substitutions(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Query<'tcx> {
    pub world_query: WorldQuery<'tcx>,
    pub filter_query: FilterQuery<'tcx>,
    pub span: Span,
}

impl<'tcx> Query<'tcx> {
    pub fn remove_substitutions(&mut self) {
        self.world_query.remove_substitutions();
        self.filter_query.remove_substitutions();
    }
}

#[derive(Debug, Clone)]
pub enum WorldQuery<'tcx> {
    Tuple(Vec<WorldQuery<'tcx>>, Span),
    Data(rustc_middle::ty::TyKind<'tcx>, rustc_ast::Mutability, Span),
    Option((Box<WorldQuery<'tcx>>, Span), Span),
}

impl<'tcx> WorldQuery<'tcx> {
    pub fn remove_substitutions(&mut self) {
        match self {
            WorldQuery::Tuple(world_querys, _) => {
                for world_query in world_querys {
                    world_query.remove_substitutions();
                }
            }
            WorldQuery::Data(kind, _, _) => {
                if let rustc_middle::ty::TyKind::Alias(_, projection) = kind {
                    let target = projection.args.first().unwrap().expect_ty().kind();

                    match target {
                        rustc_middle::ty::TyKind::Param(param)
                            if param.name == Symbol::intern("Self") =>
                        { /* Do nothing here, to avoid turning associative types into self. */ }
                        _ => *kind = *target,
                    }
                }
            }
            WorldQuery::Option(world_query, _) => world_query.0.remove_substitutions(),
        }
    }

    pub fn span(&self) -> &Span {
        match self {
            WorldQuery::Tuple(_, span)
            | WorldQuery::Data(_, _, span)
            | WorldQuery::Option(_, span) => span,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FilterQuery<'tcx> {
    Tuple(Vec<FilterQuery<'tcx>>),
    Or(Vec<FilterQuery<'tcx>>, Span),
    With(rustc_middle::ty::TyKind<'tcx>, Span),
    Without(rustc_middle::ty::TyKind<'tcx>, Span),
    Added(rustc_middle::ty::TyKind<'tcx>, Span),
    Changed(rustc_middle::ty::TyKind<'tcx>, Span),
}

impl<'tcx> FilterQuery<'tcx> {
    pub fn remove_substitutions(&mut self) {
        match self {
            FilterQuery::Tuple(filter_querys) | FilterQuery::Or(filter_querys, _) => {
                for filter_query in filter_querys {
                    filter_query.remove_substitutions();
                }
            }
            FilterQuery::With(kind, _)
            | FilterQuery::Without(kind, _)
            | FilterQuery::Added(kind, _)
            | FilterQuery::Changed(kind, _) => {
                if let rustc_middle::ty::TyKind::Alias(_, projection) = kind {
                    let target = projection.args.first().unwrap().expect_ty().kind();

                    match target {
                        rustc_middle::ty::TyKind::Param(param)
                            if param.name == Symbol::intern("Self") =>
                        { /* Do nothing here, to avoid turning associative types into self. */ }
                        _ => *kind = *target,
                    }
                }
            }
        }
    }
}
