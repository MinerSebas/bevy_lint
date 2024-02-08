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

fn test_query1(query: Query<&A, Or<()>>) {
    assert_is_system(test_query1);
    assert_eq!(query.iter().count(), 0);
}

fn test_query2(query: Query<&A, Or<(With<B>,)>>, query_check: Query<&A, With<B>>) {
    assert_is_system(test_query2);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query3(query: Query<&A, Or<(With<A>, With<B>)>>, query_check: Query<&A>) {
    assert_is_system(test_query3);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query4(
    query: Query<(&A, &C), Or<((With<A>, With<B>), With<C>)>>,
    query_check: Query<(&A, &C)>,
) {
    assert_is_system(test_query4);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query5(query: Query<&A, Or<(Or<(With<A>, With<B>)>, With<C>)>>, query_check: Query<&A>) {
    assert_is_system(test_query5);
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
    assert_is_system(test_query6);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query7(
    query: Query<(), (Or<(With<A>, With<B>)>, Added<A>)>,
    query_check: Query<(), Added<A>>,
) {
    assert_is_system(test_query7);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query8(
    query: Query<(), (Or<(Without<A>, With<B>)>, Without<A>)>,
    query_check: Query<(), Without<A>>,
) {
    assert_is_system(test_query8);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query9(mut query: Query<(&A, Or<(With<B>,)>)>, query_check: Query<&A, With<B>>) {
    assert_is_system(test_query9);
    assert_eq!(query.iter_mut().count(), query_check.iter().count());
}

fn test_query10(mut query: Query<(&A, Or<(With<A>, With<B>)>)>, query_check: Query<&A>) {
    assert_is_system(test_query10);
    assert_eq!(query.iter_mut().count(), query_check.iter().count());
}

fn negativ_test_query1(
    query: Query<&A, Or<((With<A>, With<B>), With<C>)>>,
    query_check: Query<&A, Or<(With<B>, With<C>)>>,
) {
    assert_is_system(negativ_test_query1);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn negativ_test_query2(query: Query<(), (Or<(Added<A>, With<B>)>, With<A>)>) {
    assert_is_system(negativ_test_query2);
    assert_eq!(query.iter().count(), 4);
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
        assert_is_system(Self::system_param_test);
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
        .add_systems(Startup, setup)
        .add_systems(Update, test_query1)
        .add_systems(Update, test_query2)
        .add_systems(Update, test_query3)
        .add_systems(Update, test_query4)
        .add_systems(Update, test_query5)
        .add_systems(Update, test_query6)
        .add_systems(Update, test_query7)
        .add_systems(Update, test_query8)
        .add_systems(Update, test_query9)
        .add_systems(Update, test_query10)
        .add_systems(Update, negativ_test_query1)
        .add_systems(Update, negativ_test_query2)
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
