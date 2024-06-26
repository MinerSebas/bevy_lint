#![allow(clippy::type_complexity)]
use bevy::{
    app::{App, Startup, Update},
    ecs::{
        component::Component,
        prelude::*,
        system::{assert_is_system, SystemParam},
    },
};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Default, Component)]
struct A;
#[derive(Debug, Clone, Copy, Default, Component)]
struct B;
#[derive(Debug, Clone, Copy, Default, Component)]
struct C;
#[derive(Debug, Clone, Copy, Default, Component)]
struct D<E: Component> {
    marker: PhantomData<E>,
}

fn test_query1(query: Query<&A, With<A>>, query_check: Query<&A>) {
    assert_is_system(test_query1);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query2(query: Query<(&A, &B), With<A>>, query_check: Query<(&A, &B)>) {
    assert_is_system(test_query2);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query3(query: Query<(&A, &B), With<B>>, query_check: Query<(&A, &B)>) {
    assert_is_system(test_query3);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query4(query: Query<(&A, &B), (With<A>, With<B>)>, query_check: Query<(&A, &B)>) {
    assert_is_system(test_query4);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query5(query: Query<(&A, &B), (With<A>, With<C>)>, query_check: Query<(&A, &B), With<C>>) {
    assert_is_system(test_query5);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query6(query: Query<&A, (With<A>, With<B>)>, query_check: Query<&A, With<B>>) {
    assert_is_system(test_query6);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query7(mut query: Query<&mut A, With<A>>) {
    assert_is_system(test_query7);
    assert_eq!(query.iter_mut().count(), 8);
}

fn test_query8(mut query: Query<(&mut A, &B), With<A>>) {
    assert_is_system(test_query8);
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query9(mut query: Query<(&mut A, &B), With<B>>) {
    assert_is_system(test_query9);
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query10(mut query: Query<(&mut A, &B), (With<A>, With<B>)>) {
    assert_is_system(test_query10);
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query11(mut query: Query<(&mut A, &B), (With<A>, With<C>)>) {
    assert_is_system(test_query11);
    assert_eq!(query.iter_mut().count(), 2);
}

fn test_query12(mut query: Query<&mut A, (With<A>, With<B>)>) {
    assert_is_system(test_query12);
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query13(query: Query<(), (Added<A>, With<A>)>, query_check: Query<(), Added<A>>) {
    assert_is_system(test_query13);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query14(query: Query<(), (Changed<A>, With<A>)>, query_check: Query<(), Changed<A>>) {
    assert_is_system(test_query14);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query15(query: (Query<&A, With<A>>,), query_check: Query<&A>) {
    assert_is_system(test_query15);
    assert_eq!(query.0.iter().count(), query_check.iter().count());
}

fn test_query16(query: (((((((((Query<&A, With<A>>,),),),),),),),),), query_check: Query<&A>) {
    assert_is_system(test_query16);
    assert_eq!(
        query.0 .0 .0 .0 .0 .0 .0 .0 .0.iter().count(),
        query_check.iter().count()
    );
}

fn test_query17(
    query: (
        (),
        (((
            (),
            (
                ((((Query<&A, With<A>>,), ()),),),
                (),
                (Query<&A, With<A>>, ()),
            ),
        ),),),
    ),
    query_check: Query<&A>,
) {
    assert_is_system(test_query17);
    assert_eq!(
        query.1 .0 .0 .1 .0 .0 .0 .0 .0.iter().count(),
        query_check.iter().count()
    );
    assert_eq!(
        query.1 .0 .0 .1 .2 .0.iter().count(),
        query_check.iter().count()
    );
}

fn test_query18(
    query: Query<(((((((((((&A,),),),),),),),),),),), (((((((((((With<A>,),),),),),),),),),),)>,
    query_check: Query<&A>,
) {
    assert_is_system(test_query18);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query19(
    query: Query<
        ((), ((((), (((((&A,), ()),),), (), ((),))),),)),
        ((), ((((), (((((), ()),),), (), (With<A>,))),),)),
    >,
    query_check: Query<&A>,
) {
    assert_is_system(test_query19);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query20(
    query: (
        (),
        (((
            (),
            (
                (((
                    (
                        Query<
                            ((), ((((), (((((&A,), ()),),), (), ((),))),),)),
                            ((), ((((), (((((), ()),),), (), (With<A>,))),),)),
                        >,
                    ),
                    (),
                ),),),
                (),
                (
                    Query<
                        ((), ((((), (((((&A,), ()),),), (), ((),))),),)),
                        ((), ((((), (((((), ()),),), (), (With<A>,))),),)),
                    >,
                    (),
                ),
            ),
        ),),),
    ),
    query_check: Query<&A>,
) {
    assert_is_system(test_query20);
    assert_eq!(
        query.1 .0 .0 .1 .0 .0 .0 .0 .0.iter().count(),
        query_check.iter().count()
    );
    assert_eq!(
        query.1 .0 .0 .1 .2 .0.iter().count(),
        query_check.iter().count()
    );
}

fn test_query21<E: Component>(query: Query<&E, With<E>>, query_check: Query<&E>) {
    assert_is_system(test_query21::<B>);
    assert_eq!(query.iter().count(), query_check.iter().count());
}

#[derive(SystemParam)]
struct SystemParamTest<'w, 's> {
    query1: Query<'w, 's, &'static A, With<A>>,
    query2: (
        (),
        (((
            (),
            (
                ((((Query<'w, 's, (), (Changed<A>, With<A>)>,), ()),),),
                (),
                (
                    Query<
                        'w,
                        's,
                        (((((((((((&'static A,),),),),),),),),),),),
                        (((((((((((With<A>,),),),),),),),),),),),
                    >,
                    (),
                ),
            ),
        ),),),
    ),
    query_check: Query<'w, 's, &'static A>,
}

impl<'w, 's> SystemParamTest<'w, 's> {
    fn system_param_test(system_param: SystemParamTest) {
        assert_is_system(Self::system_param_test);

        assert_eq!(
            system_param.query1.iter().count(),
            system_param.query_check.iter().count()
        );
        assert_eq!(
            system_param.query2.1 .0 .0 .1 .0 .0 .0 .0 .0.iter().count(),
            system_param.query_check.iter().count()
        );
        assert_eq!(
            system_param.query2.1 .0 .0 .1 .2 .0.iter().count(),
            system_param.query_check.iter().count()
        );
    }
}

trait TestTrait1: Component + Sized {
    type TestType: Component;

    fn test_trait1_query1(query: Query<&Self, With<Self>>, query_check: Query<&Self>) {
        assert_is_system(Self::test_trait1_query1);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query2(
        query: Query<&Self::TestType, With<Self::TestType>>,
        query_check: Query<&Self::TestType>,
    ) {
        assert_is_system(Self::test_trait1_query2);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query3(query: Query<&Self, With<Self::TestType>>, query_check: Query<&Self>) {
        assert_is_system(Self::test_trait1_query3);
        let _ = query;
        let _ = query_check;
    }
    fn test_trait1_query4(
        query: Query<&Self::TestType, With<Self>>,
        query_check: Query<&Self::TestType>,
    ) {
        assert_is_system(Self::test_trait1_query4);
        let _ = query;
        let _ = query_check;
    }
}

impl TestTrait1 for A {
    type TestType = A;
}

impl TestTrait1 for B {
    type TestType = Self;

    fn test_trait1_query1(query: Query<&Self, With<Self>>, query_check: Query<&Self>) {
        assert_is_system(Self::test_trait1_query1);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query2(
        query: Query<&Self::TestType, With<Self::TestType>>,
        query_check: Query<&Self::TestType>,
    ) {
        assert_is_system(Self::test_trait1_query2);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query3(query: Query<&Self, With<Self::TestType>>, query_check: Query<&Self>) {
        assert_is_system(Self::test_trait1_query3);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query4(
        query: Query<&Self::TestType, With<Self>>,
        query_check: Query<&Self::TestType>,
    ) {
        assert_is_system(Self::test_trait1_query4);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
}

impl TestTrait1 for C {
    type TestType = C;

    fn test_trait1_query1(query: Query<&Self, With<Self>>, query_check: Query<&Self>) {
        assert_is_system(Self::test_trait1_query1);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query2(
        query: Query<&Self::TestType, With<Self::TestType>>,
        query_check: Query<&Self::TestType>,
    ) {
        assert_is_system(Self::test_trait1_query2);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query3(query: Query<&Self, With<Self::TestType>>, query_check: Query<&Self>) {
        assert_is_system(Self::test_trait1_query3);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query4(
        query: Query<&Self::TestType, With<Self>>,
        query_check: Query<&Self::TestType>,
    ) {
        assert_is_system(Self::test_trait1_query4);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
}

trait TestTrait2: Component + Sized {
    type TestType1: Component;
    type TestType2: Component;

    fn test_trait2_query1(
        query: Query<&Self::TestType1, With<Self::TestType2>>,
        query_check: Query<&Self::TestType1>,
    ) {
        assert_is_system(Self::test_trait2_query1);
        let _ = query;
        let _ = query_check;
    }
}

impl TestTrait2 for A {
    type TestType1 = Self;
    type TestType2 = Self::TestType1;

    fn test_trait2_query1(
        query: Query<&Self::TestType1, With<Self::TestType2>>,
        query_check: Query<&Self::TestType1>,
    ) {
        assert_is_system(Self::test_trait2_query1);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
}

impl<T: Component> TestTrait2 for D<T> {
    type TestType1 = T;
    type TestType2 = Self::TestType1;

    fn test_trait2_query1(
        query: Query<&Self::TestType1, With<Self::TestType2>>,
        query_check: Query<&Self::TestType1>,
    ) {
        assert_is_system(Self::test_trait2_query1);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
}

trait TestTrait3: Component + Sized {
    fn test_trait3_query1(query: Query<&A, With<Self>>, query_check: Query<&A>) {
        assert_is_system(Self::test_trait3_query1);
        let _ = query;
        let _ = query_check;
    }
}

impl TestTrait3 for A {
    fn test_trait3_query1(query: Query<&A, With<Self>>, query_check: Query<&A>) {
        assert_is_system(Self::test_trait3_query1);
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
}

impl TestTrait3 for B {
    fn test_trait3_query1(query: Query<&A, With<Self>>, query_check: Query<&A>) {
        assert_is_system(Self::test_trait3_query1);
        assert_ne!(query.iter().count(), query_check.iter().count());
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
        .add_systems(Update, test_query11)
        .add_systems(Update, test_query12)
        .add_systems(Update, test_query13)
        .add_systems(Update, test_query14)
        .add_systems(Update, test_query15)
        .add_systems(Update, test_query16)
        .add_systems(Update, test_query17)
        .add_systems(Update, test_query18)
        .add_systems(Update, test_query19)
        .add_systems(Update, test_query20)
        .add_systems(Update, test_query21::<A>)
        .add_systems(Update, SystemParamTest::system_param_test)
        .add_systems(Update, A::test_trait1_query1)
        .add_systems(Update, A::test_trait1_query2)
        .add_systems(Update, A::test_trait1_query3)
        .add_systems(Update, A::test_trait1_query4)
        .add_systems(Update, B::test_trait1_query1)
        .add_systems(Update, B::test_trait1_query2)
        .add_systems(Update, B::test_trait1_query3)
        .add_systems(Update, B::test_trait1_query4)
        .add_systems(Update, C::test_trait1_query1)
        .add_systems(Update, C::test_trait1_query2)
        .add_systems(Update, C::test_trait1_query3)
        .add_systems(Update, C::test_trait1_query4)
        .add_systems(Update, A::test_trait2_query1)
        .add_systems(Update, A::test_trait3_query1)
        .add_systems(Update, B::test_trait3_query1)
        .add_systems(Update, D::<C>::test_trait2_query1)
        .run();
}

fn setup(mut commands: Commands) {
    let d: D<A> = D::default();

    commands.spawn((A,));
    commands.spawn((B,));
    commands.spawn((C,));
    commands.spawn((d,));
    commands.spawn((A, B));
    commands.spawn((B, C));
    commands.spawn((C, d));
    commands.spawn((A, C));
    commands.spawn((B, d));
    commands.spawn((A, d));
    commands.spawn((A, B, C));
    commands.spawn((B, C, d));
    commands.spawn((A, C, d));
    commands.spawn((A, B, d));
    commands.spawn((A, B, C, d));
}
