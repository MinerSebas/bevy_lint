use clippy_utils::{
    diagnostics::span_lint_and_then, in_macro, is_diag_trait_item, source::snippet,
};
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind, QPath};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::lint::in_external_macro;
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::{sym, Symbol};

declare_lint! {
    /// **What it does:**
    /// Checks for calls of `bevy::app::App::insert_resource` with `T::default()`.
    ///
    /// **Why is this bad?**
    /// Readability, these can be written as .init_resource<T>(), which is simpler and more concise.
    ///
    /// **Known problems:**
    /// `init_resource()` does not override already existing Resources.
    /// If you want to override a Resource with its default Values,
    /// you need to use `insert_resource` instead.
    ///
    /// **Example:**
    /// ```rust
    /// # use bevy::app::App;
    /// #[derive(Default)]
    /// struct MyResource;
    ///
    /// App::new().insert_resource(MyResource::default());
    /// ```
    /// Is better expressed with:
    /// ```rust
    /// # use bevy::app::App;
    /// #[derive(Default)]
    /// struct MyResource;
    ///
    /// App::new().init_resource::<MyResource>();
    /// ```
    pub INSERT_RESOURCE_WITH_DEFAULT,
    Warn,
    "Checks for `bevy::app::App::` on a value of type `T` with `T::default()`."
}

declare_lint_pass!(AppLintPass => [INSERT_RESOURCE_WITH_DEFAULT]);

impl<'tcx> LateLintPass<'tcx> for AppLintPass {
    fn check_expr(&mut self, cxt: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if !in_external_macro(cxt.tcx.sess, expr.span);
            if let ExprKind::MethodCall(segment, _, func_args, expr_span) = expr.kind;
            if segment.ident.name == Symbol::intern("insert_resource");
            if let rustc_middle::ty::TyKind::Adt(adt, _) = cxt.typeck_results().expr_ty(&func_args[0]).peel_refs().kind();
            let variant = adt.variants.iter().next().unwrap();
            if variant.ident.name == Symbol::intern("App");
            if let ExprKind::Call(func_expr, _) = &func_args[1].kind;
            if let ExprKind::Path(ref path) = func_expr.kind;
            if let Some(repl_def_id) = cxt.qpath_res(path, func_expr.hir_id).opt_def_id();
            if is_diag_trait_item(cxt, repl_def_id, sym::Default);
            then {
                let span = if let QPath::TypeRelative(ty, _) = path { ty.span } else { return; };

                span_lint_and_then(
                    cxt,
                    INSERT_RESOURCE_WITH_DEFAULT,
                    expr_span,
                    "initializing a Resource of type `T` with `T::default()` is better expressed using `init_resource`",
                    |diag| {
                        if !in_macro(expr_span) {
                            let suggestion = format!("init_resource::<{}>()", snippet(cxt, span, ""));
                            diag.span_suggestion(
                                expr_span,
                                "consider using",
                                suggestion,
                                Applicability::MaybeIncorrect
                            );
                        }
                    }
                );
            }
        }
    }
}
