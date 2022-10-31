#![allow(clippy::type_complexity)]
use bevy::{
    app::App,
    ecs::{prelude::*, system::assert_is_system},
};

#[derive(Debug, Component)]
struct A;
#[derive(Debug, Component)]
struct B;
#[derive(Debug, Component)]
struct C;

// TODO: Actualy test the difference between Added and Changed.

fn test_query1(_query: Query<(), (Added<A>, Changed<A>)>) {
    assert_is_system(test_query1);
}

fn test_query2(_query: Query<(), Or<(Added<A>, Changed<A>)>>) {
    assert_is_system(test_query2);
}

fn test_query3(_query: Query<(Added<A>, Changed<A>)>) {
    assert_is_system(test_query3);
}

fn test_query4(_query: Query<Or<(Added<A>, Changed<A>)>>) {
    assert_is_system(test_query4);
}

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(test_query1)
        .add_system(test_query2)
        .add_system(test_query3)
        .add_system(test_query4)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle((A,));
    commands.spawn_bundle((B,));
    commands.spawn_bundle((C,));
    commands.spawn_bundle((A, B));
    commands.spawn_bundle((B, C));
    commands.spawn_bundle((A, C));
    commands.spawn_bundle((A, B, C));
}
