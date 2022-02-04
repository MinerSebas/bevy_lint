#![allow(clippy::type_complexity)]
use bevy::{app::App, ecs::prelude::*};

#[derive(Debug, Component)]
struct A;
#[derive(Debug, Component)]
struct B;
#[derive(Debug, Component)]
struct C;

fn test_query1(mut query: Query<(&A, With<A>)>) {
    test_query1.system();
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query2(mut query: Query<(&A, Without<A>)>) {
    test_query2.system();
    assert_eq!(query.iter_mut().count(), 0);
}

fn test_query3(mut query: Query<(&A, Added<A>)>) {
    test_query3.system();
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query4(mut query: Query<(&A, Changed<A>)>) {
    test_query4.system();
    assert_eq!(query.iter_mut().count(), 4);
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
