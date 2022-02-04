use std::marker::PhantomData;

use bevy::{
    ecs::prelude::*,
    transform::prelude::{GlobalTransform, Transform},
};

#[derive(Debug, Component)]
struct A;
#[derive(Debug, Component)]
struct B;
#[derive(Debug, Component)]
struct C;
#[derive(Debug, Component)]
struct D<E: Component> {
    marker: PhantomData<E>,
}

#[derive(Bundle)]
struct BundleWithoutGlobalTransform {
    transform: Transform,
}

#[derive(Bundle)]
struct BundleWithoutTransform {
    global_transform: GlobalTransform,
}

#[derive(Bundle)]
struct BundleWithBothTransforms {
    transform: Transform,
    global_transform: GlobalTransform,
}

#[derive(Bundle)]
struct ComplexBundleWithoutGlobalTransform {
    transform: Transform,
    a: A,
    b: B,
    c: C,
    d: D<GlobalTransform>,
}

#[derive(Bundle)]
struct ComplexBundleWithoutTransform {
    d: D<Transform>,
    c: C,
    global_transform: GlobalTransform,
    a: A,
    b: B,
}

fn main() {
    // Nothing to test here at runtime.
}
