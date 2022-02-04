#![feature(rustc_private)]
#![feature(let_else)]
//#![warn(unused_extern_crates)]
#![recursion_limit = "1000"]

dylint_linting::dylint_library!();

extern crate rustc_ast;
//extern crate rustc_ast_pretty;
//extern crate rustc_attr;
//extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
//extern crate rustc_hir_pretty;
//extern crate rustc_index;
//extern crate rustc_infer;
//extern crate rustc_lexer;
extern crate rustc_lint;
extern crate rustc_middle;
//extern crate rustc_mir;
//extern crate rustc_parse;
//extern crate rustc_parse_format;
extern crate rustc_session;
extern crate rustc_span;
//extern crate rustc_target;
//extern crate rustc_trait_selection;
//extern crate rustc_typeck;

mod app_lints;
mod bevy_paths;
mod bundle_lints;
mod mixed_ty;
mod system_lints;

pub use app_lints::INSERT_RESOURCE_WITH_DEFAULT;
pub use bundle_lints::BUNDLE_WITH_INCOMPLETE_TRANSFORMS;
pub use system_lints::query_lints::{
    EMPTY_QUERY, FILTER_IN_WORLD_QUERY, UNNECESSARY_ADDED, UNNECESSARY_CHANGED, UNNECESSARY_OPTION,
    UNNECESSARY_OR, UNNECESSARY_WITH,
};

#[no_mangle]
#[doc(hidden)]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[
        INSERT_RESOURCE_WITH_DEFAULT,
        BUNDLE_WITH_INCOMPLETE_TRANSFORMS,
        EMPTY_QUERY,
        FILTER_IN_WORLD_QUERY,
        UNNECESSARY_ADDED,
        UNNECESSARY_CHANGED,
        UNNECESSARY_OPTION,
        UNNECESSARY_OR,
        UNNECESSARY_WITH,
    ]);
    lint_store.register_late_pass(|| Box::new(app_lints::AppLintPass));
    lint_store.register_late_pass(|| Box::new(bundle_lints::BundleLintPass));
    lint_store.register_late_pass(|| Box::new(system_lints::SystemLintPass));
}

#[test]
fn ui() {
    dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
}
