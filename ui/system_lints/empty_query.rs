#![allow(clippy::type_complexity)]
use bevy::{
    app::{App, Startup, Update},
    ecs::{
        prelude::*,
        system::{assert_is_system, SystemParam},
    },
};

#[derive(Debug, Component)]
struct A;
#[derive(Debug, Component)]
struct B;
#[derive(Debug, Component)]
struct C;

fn test_query1(query: Query<&A, Without<A>>) {
    assert_is_system(test_query1);
    assert_eq!(query.iter().count(), 0);
}

fn test_query2(mut query: Query<&mut A, Without<A>>) {
    assert_is_system(test_query2);
    assert_eq!(query.iter_mut().count(), 0);
}

fn test_query3(query: Query<&A, (Without<A>, With<B>)>) {
    assert_is_system(test_query3);
    assert_eq!(query.iter().count(), 0);
}

fn test_query4(query: Query<&B, (With<A>, Without<B>)>) {
    assert_is_system(test_query4);
    assert_eq!(query.iter().count(), 0);
}

fn test_query5(query: Query<(), (Without<A>, Added<A>)>) {
    assert_is_system(test_query5);
    assert_eq!(query.iter().count(), 0);
}

fn test_query6(query: Query<(), (Changed<A>, Without<A>)>) {
    assert_is_system(test_query6);
    assert_eq!(query.iter().count(), 0);
}

#[derive(SystemParam)]
struct SystemParamTest<'w, 's> {
    query1: Query<'w, 's, &'static A, Without<A>>,
    query2: (
        (),
        (((
            (),
            (
                (((
                    (
                        Query<
                            'w,
                            's,
                            (Option<&'static mut C>, &'static B),
                            (Without<C>, Without<B>),
                        >,
                    ),
                    (),
                ),),),
                (),
                (
                    Query<
                        'w,
                        's,
                        (),
                        ((), ((((), ((((With<A>,), ()),),), (), (Without<A>, ())),),)),
                    >,
                    (),
                ),
            ),
        ),),),
    ),
}

impl<'w, 's> SystemParamTest<'w, 's> {
    fn system_param_test(mut system_param: SystemParamTest) {
        assert_is_system(Self::system_param_test);
        assert_eq!(system_param.query1.iter().count(), 0);
        assert_eq!(
            system_param
                .query2
                .1
                 .0
                 .0
                 .1
                 .0
                 .0
                 .0
                 .0
                 .0
                .iter_mut()
                .count(),
            0
        );
        assert_eq!(system_param.query2.1 .0 .0 .1 .2 .0.iter().count(), 0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, test_query1)
        .add_systems(Update, test_query2)
        .add_systems(Update, test_query3)
        .add_systems(Update, test_query4)
        .add_systems(Update, test_query5)
        .add_systems(Update, test_query6)
        .add_systems(Update, SystemParamTest::system_param_test)
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
