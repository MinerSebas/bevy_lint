#![allow(clippy::type_complexity)]
#![allow(dead_code)]
use bevy::{
    app::{App, Plugin},
    ecs::system::Resource,
};

#[derive(Debug, Default, Resource)]
struct A(u8);
#[derive(Debug, Default, Resource)]
struct B(u8);

struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(A::default())
            .insert_resource(B::default());
        app.insert_non_send_resource(A::default())
            .insert_non_send_resource(B::default());
    }
}

fn main() {
    App::new().insert_resource(A::default());
    App::new().insert_resource(B::default());
    App::new().insert_resource(A(0)).insert_resource(B(0));
    App::new()
        .insert_resource(A::default())
        .insert_resource(B::default());
    App::new()
        .insert_resource(A::default())
        .insert_resource(B(0));
    App::new()
        .insert_resource(A(0))
        .insert_resource(B::default());

    App::new().insert_non_send_resource(A::default());
    App::new().insert_non_send_resource(B::default());
    App::new()
        .insert_non_send_resource(A)
        .insert_non_send_resource(B);
    App::new()
        .insert_non_send_resource(A::default())
        .insert_non_send_resource(B::default());
    App::new()
        .insert_non_send_resource(A::default())
        .insert_non_send_resource(B);
    App::new()
        .insert_non_send_resource(A)
        .insert_non_send_resource(B::default());

    App::new().add_plugins(TestPlugin);
}
