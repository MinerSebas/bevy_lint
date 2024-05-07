#![allow(clippy::type_complexity)]
use bevy::{
    app::{App, Startup, Update},
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

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, test_query1)
        .add_systems(Update, test_query2)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((A,));
    commands.spawn((B,));
    commands.spawn((C,));
    commands.spawn((A, B));
    commands.spawn((B, C));
    commands.spawn((A, C));
    commands.spawn((A, B, C));
}
