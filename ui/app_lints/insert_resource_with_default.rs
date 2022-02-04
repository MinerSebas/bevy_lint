#![allow(clippy::type_complexity)]
use bevy::app::{App, Plugin};

#[derive(Debug, Default)]
struct A;
#[derive(Debug, Default)]
struct B;

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
    App::new().insert_resource(A).insert_resource(B);
    App::new()
        .insert_resource(A::default())
        .insert_resource(B::default());
    App::new().insert_resource(A::default()).insert_resource(B);
    App::new().insert_resource(A).insert_resource(B::default());

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

    App::new().add_plugin(TestPlugin);
}
