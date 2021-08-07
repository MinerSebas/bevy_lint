#![allow(dead_code)]
use bevy::ecs::{prelude::*, system::SystemParam};

fn main() {}

struct A;
struct B;
struct C;

fn test_query1(_query: Query<&A, Without<A>>) {
    test_query1.system();
}
fn test_query2(_query: Query<&mut A, Without<A>>) {
    test_query2.system();
}
fn test_query3(_query: Query<&A, (Without<A>, With<B>)>) {
    test_query3.system();
}
fn test_query4(_query: Query<&B, (With<A>, Without<B>)>) {
    test_query4.system();
}

fn test_query5(_query: Query<Option<&A>, Without<A>>) {
    test_query5.system();
}
fn test_query6(_query: Query<Option<&mut A>, Without<A>>) {
    test_query6.system();
}
fn test_query7(_query: Query<Option<&A>, (Without<A>, With<B>)>) {
    test_query7.system();
}
fn test_query8(_query: Query<Option<&B>, (With<A>, Without<B>)>) {
    test_query8.system();
}

fn test_query9(_query: Query<(), (Without<A>, Added<A>)>) {
    test_query7.system();
}
fn test_query10(_query: Query<(), (Changed<A>, Without<A>)>) {
    test_query8.system();
}

#[derive(SystemParam)]
struct SystemParamTest<'a> {
    query1: Query<'a, &'static A, Without<A>>,
    query2: (
        (),
        (((
            (),
            (
                (((
                    (Query<'a, (Option<&'static mut A>, &'static B), (Without<A>, Without<B>)>,),
                    (),
                ),),),
                (),
                (Query<'a, (), (Or<(With<A>, With<B>)>, Without<A>)>, ()),
            ),
        ),),),
    ),
}
