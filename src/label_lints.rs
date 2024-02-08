use std::collections::HashMap;

use clippy_utils::{diagnostics::span_lint, get_trait_def_id, ty::implements_trait};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::Symbol;

use crate::bevy_paths::{
    AMBIGUITY_SET_LABEL, APP_LABEL, RUN_CRITERIA_LABEL, STAGE_LABEL, SYSTEM_LABEL,
};

declare_lint! {
    /// **What it does:**
    /// Checks for types that implement more than one Label.
    ///
    /// Checked for Lables:
    /// - [AppLabel](https://docs.rs/bevy/latest/bevy/app/trait.AppLabel.html)
    /// - [StageLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.StageLabel.html)
    /// - [SystemLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.SystemLabel.html)
    /// - [RunCriteriaLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.RunCriteriaLabel.html)
    /// - [AmbiguitySetLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.AmbiguitySetLabel.html)
    ///
    /// **Why is this bad?**
    /// This can introduce similiar Bugs to `&str` Labels that contain Typos.
    ///
    /// **Known problems:**
    /// None
    ///
    /// **Example:**
    /// ```rust
    /// # use bevy::prelude::*;
    /// # fn some_system() {};
    /// # fn conflicting_system() {};
    /// #
    /// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel, AmbiguitySetLabel)]
    /// struct SomeLabel;
    ///
    /// App::new()
    ///     .add_system(some_system.label(SomeLabel))
    ///     .add_system(conflicting_system.in_ambiguity_set(SomeLabel)); // Will still report ambiguity.
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// # use bevy::prelude::*;
    /// # fn some_system() {};
    /// # fn conflicting_system() {};
    /// #
    /// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    /// struct SomeSystemLabel;
    /// #[derive(Debug, Hash, PartialEq, Eq, Clone, AmbiguitySetLabel)]
    /// struct SomeAmbiguityLabel;
    ///
    /// App::new()
    ///	    .add_system(some_system.label(SomeSystemLabel).in_ambiguity_set(SomeAmbiguityLabel))
    ///	    .add_system(conflicting_system.in_ambiguity_set(SomeAmbiguityLabel));
    /// ```
    pub MULTIPLE_LABELS_ON_TYPE,
    Warn,
    "Checks for types that implement more than one Label."
}

declare_lint! {
    /// **What it does:**
    /// Checks for cases where a `&str` is used as a Label.
    ///
    /// Checked for Lables:
    /// - [SystemLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.SystemLabel.html)
    /// - [RunCriteriaLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.RunCriteriaLabel.html)
    /// - [AmbiguitySetLabel](https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.AmbiguitySetLabel.html)
    ///
    /// **Why is this bad?**
    /// Using strings is very suspicalbe to typos that wont be catched at compiletime.
    ///
    /// **Known problems:**
    /// None
    ///
    /// **Example:**
    /// ```rust
    /// # use bevy::prelude::*;
    /// # fn some_system() {};
    /// # fn other_system() {};
    /// #
    /// App::new()
    ///     .add_system(some_system.label("some_label"))
    ///     .add_system(other_system.after("some_lobel")); // Label with typo
    /// ```
    ///
    /// Instead, a user should use an Enum/Unit Struct that derives the Label:
    ///
    /// ```rust
    /// # use bevy::prelude::*;
    /// # fn some_system() {};
    /// # fn other_system() {};
    /// #
    /// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
    /// struct SomeLabel;
    ///
    /// App::new()
    ///     .add_system(some_system.label(SomeLabel))
    ///     .add_system(other_system.after(SomeLabel));
    /// ```
    pub STR_LABEL,
    Warn,
    "Checks for cases where a `&str` is used as a Label."
}

declare_lint_pass!(LabelLintPass => [MULTIPLE_LABELS_ON_TYPE, STR_LABEL]);

impl<'tcx> LateLintPass<'tcx> for LabelLintPass {
    fn check_expr(&mut self, ctx: &LateContext<'tcx>, expr: &'tcx rustc_hir::Expr<'tcx>) {
        match expr.kind {
            rustc_hir::ExprKind::MethodCall(..) => (),
            _ => return,
        }

        // Wrap function names in a closure, to avoid unnecessary interning.
        let lookup: [(_, fn() -> Vec<(Symbol, &'static [usize])>); 5] = [
            (Symbol::intern("RunCriteriaDescriptor"), || {
                vec![
                    (Symbol::intern("label"), &[1]),
                    (Symbol::intern("before"), &[1]),
                    (Symbol::intern("after"), &[1]),
                    (Symbol::intern("label_discard_if_duplicate"), &[1]),
                    // TODO: "pipe" is a expr of kind "Call" and not "MethodCall"
                    //(Symbol::intern("pipe"), &[0]),
                ]
            }),
            /*(Symbol::intern("BoxedSystem"), || {
                vec![
                    (Symbol::intern("label"), &[1]),
                    (Symbol::intern("before"), &[1]),
                    (Symbol::intern("after"), &[1]),
                    (Symbol::intern("in_ambiguity_set"), &[1]),
                    (Symbol::intern("label_discard_if_duplicate"), &[1]),
                ]
            }),*/
            (Symbol::intern("ParallelSystemDescriptor"), || {
                vec![
                    (Symbol::intern("label"), &[1]),
                    (Symbol::intern("before"), &[1]),
                    (Symbol::intern("after"), &[1]),
                    (Symbol::intern("in_ambiguity_set"), &[1]),
                ]
            }),
            (Symbol::intern("ExclusiveSystemDescriptor"), || {
                vec![
                    (Symbol::intern("label"), &[1]),
                    (Symbol::intern("before"), &[1]),
                    (Symbol::intern("after"), &[1]),
                    (Symbol::intern("in_ambiguity_set"), &[1]),
                ]
            }),
            (Symbol::intern("SystemSet"), || {
                vec![
                    (Symbol::intern("label"), &[1]),
                    (Symbol::intern("before"), &[1]),
                    (Symbol::intern("after"), &[1]),
                    (Symbol::intern("in_ambiguity_set"), &[1]),
                ]
            }),
            (Symbol::intern("App"), || {
                vec![
                    (Symbol::intern("add_stage"), &[1]),
                    (Symbol::intern("add_stage_before"), &[1, 2]),
                    (Symbol::intern("add_stage_after"), &[1, 2]),
                    (Symbol::intern("add_startup_stage"), &[1]),
                    (Symbol::intern("add_startup_stage_before"), &[1, 2]),
                    (Symbol::intern("add_startup_stage_after"), &[1, 2]),
                    (Symbol::intern("stage"), &[1]),
                    (Symbol::intern("add_system_to_stage"), &[1]),
                    (Symbol::intern("add_system_set_to_stage"), &[1]),
                    (Symbol::intern("add_startup_system_to_stage"), &[1]),
                    (Symbol::intern("add_startup_system_set_to_stage"), &[1]),
                    (Symbol::intern("add_sub_app"), &[1]),
                    (Symbol::intern("sub_app_mut"), &[1]),
                    (Symbol::intern("sub_app"), &[1]),
                    // TODO: Dont work as their return type is Result<App, _>, not App
                    //(Symbol::intern("get_sub_app_mut"), &[1]),
                    //(Symbol::intern("get_sub_app"), &[1]),
                ]
            }),
        ];

        let lookup: HashMap<_, _> = lookup.into();

        let ty = ctx.typeck_results().expr_ty(expr);

        //dbg!(expr);
        //dbg!(ty);
        //dbg!(ty.kind());

        check_for_label(ctx, expr, &ty, lookup);
    }

    fn check_item(&mut self, ctx: &LateContext<'tcx>, item: &'tcx rustc_hir::Item<'tcx>) {
        match item.kind {
            rustc_hir::ItemKind::Enum(_, _) | rustc_hir::ItemKind::Struct(_, _) => (),
            _ => return,
        }

        let Some(stage_def_id) = get_trait_def_id(ctx, STAGE_LABEL) else {
            return;
        };
        let Some(system_def_id) = get_trait_def_id(ctx, SYSTEM_LABEL) else {
            // If the "STAGE_LABEL" trait exits then all other traits must also exist.
            unreachable!()
        };
        let Some(criteria_def_id) = get_trait_def_id(ctx, RUN_CRITERIA_LABEL) else {
            unreachable!()
        };
        let Some(ambiguity_def_id) = get_trait_def_id(ctx, AMBIGUITY_SET_LABEL) else {
            unreachable!()
        };

        let app_def_id = get_trait_def_id(ctx, APP_LABEL);

        let imp_stage = implements_trait(
            ctx,
            ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
            stage_def_id,
            &[],
        );
        let imp_system = implements_trait(
            ctx,
            ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
            system_def_id,
            &[],
        );
        let imp_criteria = implements_trait(
            ctx,
            ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
            criteria_def_id,
            &[],
        );
        let imp_ambiguity = implements_trait(
            ctx,
            ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
            ambiguity_def_id,
            &[],
        );
        let imp_app = app_def_id.is_some()
            && implements_trait(
                ctx,
                ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
                app_def_id.unwrap(),
                &[],
            );

        let imp_count = imp_stage as usize
            + imp_system as usize
            + imp_criteria as usize
            + imp_ambiguity as usize
            + imp_app as usize;

        if imp_count >= 2 {
            let labels = {
                let mut count = 0;
                let mut string = String::new();

                let mut connector = |string: &mut String| {
                    if count + 1 == imp_count {
                        string.push_str(" and ");
                    } else if count != 0 {
                        string.push_str(", ");
                    }
                    count += 1;
                };

                if imp_stage {
                    connector(&mut string);
                    string.push_str(STAGE_LABEL.last().unwrap());
                }
                if imp_system {
                    connector(&mut string);
                    string.push_str(SYSTEM_LABEL.last().unwrap());
                }
                if imp_criteria {
                    connector(&mut string);
                    string.push_str(RUN_CRITERIA_LABEL.last().unwrap());
                }
                if imp_ambiguity {
                    connector(&mut string);
                    string.push_str(AMBIGUITY_SET_LABEL.last().unwrap());
                }
                if imp_app {
                    connector(&mut string);
                    string.push_str(APP_LABEL.last().unwrap());
                }
                string
            };

            span_lint(
                ctx,
                MULTIPLE_LABELS_ON_TYPE,
                item.span,
                format!("Type implements the following Labels: {}", labels).as_str(),
            );
        }
    }
}

fn check_for_label(
    ctx: &LateContext,
    expr: &rustc_hir::Expr,
    ty: &rustc_middle::ty::Ty,
    lookup: HashMap<Symbol, fn() -> Vec<(Symbol, &'static [usize])>>,
) {
    match ty.kind() {
        rustc_middle::ty::TyKind::Ref(_, ty, _) => {
            check_for_label(ctx, expr, ty, lookup);
        }
        rustc_middle::ty::TyKind::Adt(adt_def, _) => {
            if adt_def.is_enum() {
                return;
            }

            let struct_name = adt_def
                .variants()
                .iter()
                .next()
                .unwrap()
                .ident(ctx.tcx)
                .name;

            if let Some(funcs) = lookup.get(&struct_name) {
                if let rustc_hir::ExprKind::MethodCall(segment, object, func_args, _) = expr.kind {
                    let expressions = funcs()
                        .iter()
                        .find_map(|(method_name, indexes)| {
                            if *method_name == segment.ident.name {
                                Some(*indexes)
                            } else {
                                None
                            }
                        })
                        .into_iter()
                        .flatten()
                        .map(|index| {
                            if *index == 0 {
                                object
                            } else {
                                &func_args[index - 1]
                            }
                        });

                    for expr in expressions {
                        if let rustc_hir::ExprKind::Lit(ref lit) = expr.kind {
                            if let rustc_ast::LitKind::Str(_, _) = lit.node {
                                span_lint(ctx, STR_LABEL, lit.span, "String used as Label");
                            }
                        }
                    }
                }
            }
        }
        _ => (),
    }
}
