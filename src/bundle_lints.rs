use clippy_utils::{
    diagnostics::span_lint,
    get_trait_def_id,
    ty::{implements_trait, match_type},
};
use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint, declare_lint_pass};

use crate::{
    bevy_paths::{BUNDLE, GLOBAL_TRANSFORM, TRANSFORM},
    mixed_ty::MixedTy,
};

declare_lint! {
    /// **What it does:**
    /// Checks for Bundles that contain a `Transform` or `GlobalTransform`, but not the other.
    ///
    /// **Why is this bad?**
    /// When creating a Hierachy of Entitys, Bevy excpects every Entity to have
    /// both a `Transform` and `GlobalTransform`.
    ///
    /// If this is not the case then the `GlobalTransform` of Children will not be updated.
    ///
    /// **Known problems:**  None.
    ///
    /// **Example:**
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// # use bevy::transform::prelude::*;
    /// #
    /// #[derive(Bundle)]
    /// struct CustomBundle {
    ///    transform: Transform,
    /// }
    /// ```
    /// Is better expressed with:
    /// ```rust
    /// # use bevy::ecs::prelude::*;
    /// # use bevy::transform::prelude::*;
    /// #
    /// #[derive(Bundle)]
    /// struct CustomBundle {
    ///    transform: Transform,
    ///    global_transform: GlobalTransform,
    /// }
    /// ```
    pub BUNDLE_WITH_INCOMPLETE_TRANSFORMS,
    Warn,
    "Checks for `bevy::app::App::` on a value of type `T` with `T::default()`."
}

declare_lint_pass!(BundleLintPass => [BUNDLE_WITH_INCOMPLETE_TRANSFORMS]);

impl<'tcx> LateLintPass<'tcx> for BundleLintPass {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        match item.kind {
            ItemKind::Struct(_, _) => (),
            _ => return,
        };

        let bundle_def_id = if let Some(def_id) = get_trait_def_id(ctx, BUNDLE) {
            def_id
        } else {
            return;
        };

        if !implements_trait(ctx, ctx.tcx.type_of(item.def_id), bundle_def_id, &[]) {
            return;
        }

        let mut contains_transform = false;
        let mut contains_global_transform = false;

        for field in MixedTy::fields_from_struct_item(ctx, item).unwrap() {
            if match_type(ctx, field.middle, TRANSFORM) {
                contains_transform = true;
            } else if match_type(ctx, field.middle, GLOBAL_TRANSFORM) {
                contains_global_transform = true;
            }

            if contains_transform && contains_global_transform {
                return;
            }
        }

        match (contains_transform, contains_global_transform) {
            (true, false) => {
                span_missing_transform_lint(ctx, item, MissingTransformVariant::Transform);
            }
            (false, true) => {
                span_missing_transform_lint(ctx, item, MissingTransformVariant::GlobalTransform);
            }
            _ => (),
        }
    }
}

enum MissingTransformVariant {
    Transform,
    GlobalTransform,
}

impl MissingTransformVariant {
    fn missing(&self) -> &'static str {
        match self {
            MissingTransformVariant::Transform => "Transform",
            MissingTransformVariant::GlobalTransform => "GlobalTransform",
        }
    }

    fn present(&self) -> &'static str {
        match self {
            MissingTransformVariant::Transform => "GlobalTransform",
            MissingTransformVariant::GlobalTransform => "Transform",
        }
    }
}

fn span_missing_transform_lint(
    ctx: &LateContext,
    bundle: &rustc_hir::Item,
    missing: MissingTransformVariant,
) {
    span_lint(
        ctx,
        BUNDLE_WITH_INCOMPLETE_TRANSFORMS,
        bundle.span,
        format!(
            "This Bundle contains the \"{}\" Component, but is missing the \"{}\" Component.",
            missing.present(),
            missing.missing()
        )
        .as_str(),
    );
}
