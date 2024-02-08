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

        let Some(bundle_def_id) = get_trait_def_id(ctx, BUNDLE) else {
            return;
        };

        if !implements_trait(
            ctx,
            ctx.tcx.type_of(item.owner_id.def_id).skip_binder(),
            bundle_def_id,
            &[],
        ) {
            return;
        }

        let mut contains_transform = false;
        let mut contains_global_transform = false;

        for field in MixedTy::fields_from_struct_item(ctx, item).iter().flatten() {
            if match_type(ctx, field.middle, TRANSFORM) {
                contains_transform = true;
            } else if match_type(ctx, field.middle, GLOBAL_TRANSFORM) {
                contains_global_transform = true;
            } else {
                continue;
            }

            if contains_transform && contains_global_transform {
                return;
            }
        }

        let msg = match (contains_transform, contains_global_transform) {
            (true, false) => {
                "This Bundle contains the \"GlobalTransform\" Component, but is missing the \"Transform\" Component."
            }
            (false, true) => {
                "This Bundle contains the \"Transform\" Component, but is missing the \"GlobalTransform\" Component."
            }
            _ => return,
        };

        span_lint(ctx, BUNDLE_WITH_INCOMPLETE_TRANSFORMS, item.span, msg);
    }
}
