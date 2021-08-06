#![allow(dead_code)]
use bevy::ecs::{prelude::*, system::SystemParam};

fn main() {}

struct A;
struct B;
struct C;

fn test_query1(_query: Query<Option<&A>, With<A>>) {
    test_query1.system();
}
fn test_query2(_query: Query<Option<&mut A>, With<A>>) {
    test_query2.system();
}
fn test_query3(_query: Query<Option<&A>, (With<A>, With<B>)>) {
    test_query3.system();
}
fn test_query4(_query: Query<Option<&mut A>, (With<A>, With<B>)>) {
    test_query4.system();
}
fn test_query5(_query: Query<Option<&B>, (With<A>, With<B>)>) {
    test_query5.system();
}
fn test_query6(_query: Query<Option<&mut B>, (With<A>, With<B>)>) {
    test_query6.system();
}

fn test_query7(_query: Query<(Option<&A>, &B), With<A>>) {
    test_query7.system();
}
fn test_query8(_query: Query<(&A, Option<&B>), With<B>>) {
    test_query8.system();
}

fn test_query9(_query: Query<(Option<&A>, &B), Added<A>>) {
    test_query9.system();
}
fn test_query10(_query: Query<(&A, Option<&B>), Changed<B>>) {
    test_query10.system();
}

// This may not trigger the Lint
fn test_query11(_query: Query<Option<&A>, Or<(With<A>, With<B>)>>) {
    test_query11.system();
}

#[derive(SystemParam)]
struct SystemParamTest<'a> {
    query1: Query<'a, Option<&'static A>, With<A>>,
    query2: (
        (),
        (((
            (),
            (
                (((
                    (
                        Query<
                            'a,
                            (Option<&'static mut A>, Option<&'static B>),
                            (Changed<A>, With<B>),
                        >,
                    ),
                    (),
                ),),),
                (),
                (
                    Query<
                        'a,
                        (Option<&'static mut A>, Option<&'static C>),
                        (Or<(With<A>, With<B>)>, With<C>),
                    >,
                    (),
                ),
            ),
        ),),),
    ),
}
