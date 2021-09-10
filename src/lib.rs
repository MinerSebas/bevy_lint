#![feature(rustc_private)]
//#![warn(unused_extern_crates)]

dylint_linting::dylint_library!();

extern crate rustc_ast;
//extern crate rustc_ast_pretty;
//extern crate rustc_attr;
//extern crate rustc_data_structures;
//extern crate rustc_errors;
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

mod bevy_paths;
mod mixed_ty;
mod system_lints;

pub use system_lints::query_lints::{
    EMPTY_QUERY, UNNECESSARY_OPTION, UNNECESSARY_OR, UNNECESSARY_WITH,
};

#[no_mangle]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[
        EMPTY_QUERY,
        UNNECESSARY_OPTION,
        UNNECESSARY_OR,
        UNNECESSARY_WITH,
    ]);
    lint_store.register_late_pass(|| Box::new(system_lints::SystemLintPass));
}

#[test]
fn ui() {
    dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
}
