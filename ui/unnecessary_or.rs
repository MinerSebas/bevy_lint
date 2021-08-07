#![allow(dead_code)]
use bevy::ecs::{prelude::*, system::SystemParam};

fn main() {}

struct A;
struct B;
struct C;

fn test_query1(_query: Query<&A, Or<()>>) {
    test_query1.system();
}
fn test_query2(_query: Query<&A, Or<(With<B>,)>>) {
    test_query2.system();
}

#[derive(SystemParam)]
struct SystemParamTest<'a> {
    query1: Query<'a, &'static A, Or<()>>,
    query2: (
        (),
        (((
            (),
            (
                ((((Query<'a, (), (Changed<A>, Or<(With<A>,)>)>,), ()),),),
                (),
                (Query<'a, &'static mut A, (Or<(With<B>,)>, With<C>)>, ()),
            ),
        ),),),
    ),
}
