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

fn test_query1(query: Query<&A, Or<()>>) {
    test_query1.system();
    assert_eq!(query.iter().count(), 0);
}

fn test_query2(query: Query<&A, Or<(With<B>,)>>, query_check: Query<&A, With<B>>) {
    test_query2.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query3(query: Query<&A, Or<(With<A>, With<B>)>>, query_check: Query<&A>) {
    test_query3.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query4(
    query: Query<(&A, &C), Or<((With<A>, With<B>), With<C>)>>,
    query_check: Query<(&A, &C)>,
) {
    test_query4.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query5(query: Query<&A, Or<(Or<(With<A>, With<B>)>, With<C>)>>, query_check: Query<&A>) {
    test_query5.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query6(
    query: Query<
        &A,
        Or<(
            ((((((((((((Or<(With<A>, With<B>)>,),),),),),),),),),),),),
            With<C>,
        )>,
    >,
    query_check: Query<&A>,
) {
    test_query6.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query7(
    query: Query<(), (Or<(With<A>, With<B>)>, Added<A>)>,
    query_check: Query<(), Added<A>>,
) {
    test_query7.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query8(
    query: Query<(), (Or<(Without<A>, With<B>)>, Without<A>)>,
    query_check: Query<(), Without<A>>,
) {
    test_query8.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn negativ_test_query1(
    query: Query<&A, Or<((With<A>, With<B>), With<C>)>>,
    query_check: Query<&A, Or<(With<B>, With<C>)>>,
) {
    negativ_test_query1.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn negativ_test_query2(query: Query<(), (Or<(Added<A>, With<B>)>, With<A>)>) {
    negativ_test_query2.system();
    assert_eq!(query.iter().count(), 6);
}

#[derive(SystemParam)]
struct SystemParamTest<'w, 's> {
    query1: Query<'w, 's, &'static A, Or<()>>,
    query2: (
        (),
        (((
            (),
            (
                ((((Query<'w, 's, (), (Changed<A>, Or<(With<A>,)>)>,), ()),),),
                (),
                (Query<'w, 's, &'static A, (Or<(With<B>,)>, With<C>)>, ()),
            ),
        ),),),
    ),
}

impl<'w, 's> SystemParamTest<'w, 's> {
    fn system_param_test(
        system_param: SystemParamTest,
        query_check1: Query<(), Changed<A>>,
        query_check2: Query<&A, (With<B>, With<C>)>,
    ) {
        Self::system_param_test.system();
        assert_eq!(system_param.query1.iter().count(), 0);
        assert_eq!(
            system_param.query2.1 .0 .0 .1 .0 .0 .0 .0 .0.iter().count(),
            query_check1.iter().count()
        );
        assert_eq!(
            system_param.query2.1 .0 .0 .1 .2 .0.iter().count(),
            query_check2.iter().count()
        );
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
        .add_system(test_query7)
        .add_system(test_query8)
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
