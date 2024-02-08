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

fn test_query1(mut query: Query<(&A, With<A>)>) {
    assert_is_system(test_query1);
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query2(mut query: Query<(&A, Without<A>)>) {
    assert_is_system(test_query2);
    assert_eq!(query.iter_mut().count(), 0);
}

fn test_query3(mut query: Query<(&A, Added<A>)>) {
    assert_is_system(test_query3);
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query4(mut query: Query<(&A, Changed<A>)>) {
    assert_is_system(test_query4);
    assert_eq!(query.iter_mut().count(), 4);
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, test_query1)
        .add_systems(Update, test_query2)
        .add_systems(Update, test_query3)
        .add_systems(Update, test_query4)
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
