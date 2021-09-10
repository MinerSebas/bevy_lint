#![allow(dead_code)]
#![allow(clippy::type_complexity)]
use bevy::{
    app::App,
    ecs::{prelude::*, system::SystemParam},
};

#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct C;

fn test_query1(query: Query<&A, Without<A>>) {
    test_query1.system();
    assert_eq!(query.iter().count(), 0);
}

fn test_query2(mut query: Query<&mut A, Without<A>>) {
    test_query2.system();
    assert_eq!(query.iter_mut().count(), 0);
}

fn test_query3(query: Query<&A, (Without<A>, With<B>)>) {
    test_query3.system();
    assert_eq!(query.iter().count(), 0);
}

fn test_query4(query: Query<&B, (With<A>, Without<B>)>) {
    test_query4.system();
    assert_eq!(query.iter().count(), 0);
}

fn test_query5(query: Query<(), (Without<A>, Added<A>)>) {
    test_query5.system();
    assert_eq!(query.iter().count(), 0);
}

fn test_query6(query: Query<(), (Changed<A>, Without<A>)>) {
    test_query6.system();
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
        Self::system_param_test.system();
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
        .add_startup_system(setup)
        .add_system(test_query1)
        .add_system(test_query2)
        .add_system(test_query3)
        .add_system(test_query4)
        .add_system(test_query5)
        .add_system(test_query6)
        .add_system(SystemParamTest::system_param_test)
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
