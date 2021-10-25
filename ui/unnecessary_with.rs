#![allow(clippy::type_complexity)]
use bevy::{
    app::App,
    ecs::{component::Component, prelude::*, system::SystemParam},
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
    test_query1.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query2(query: Query<(&A, &B), With<A>>, query_check: Query<(&A, &B)>) {
    test_query2.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query3(query: Query<(&A, &B), With<B>>, query_check: Query<(&A, &B)>) {
    test_query3.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query4(query: Query<(&A, &B), (With<A>, With<B>)>, query_check: Query<(&A, &B)>) {
    test_query4.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query5(query: Query<(&A, &B), (With<A>, With<C>)>, query_check: Query<(&A, &B), With<C>>) {
    test_query5.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query6(query: Query<&A, (With<A>, With<B>)>, query_check: Query<&A, With<B>>) {
    test_query6.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query7(mut query: Query<&mut A, With<A>>) {
    test_query7.system();
    assert_eq!(query.iter_mut().count(), 8);
}

fn test_query8(mut query: Query<(&mut A, &B), With<A>>) {
    test_query8.system();
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query9(mut query: Query<(&mut A, &B), With<B>>) {
    test_query9.system();
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query10(mut query: Query<(&mut A, &B), (With<A>, With<B>)>) {
    test_query10.system();
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query11(mut query: Query<(&mut A, &B), (With<A>, With<C>)>) {
    test_query11.system();
    assert_eq!(query.iter_mut().count(), 2);
}

fn test_query12(mut query: Query<&mut A, (With<A>, With<B>)>) {
    test_query12.system();
    assert_eq!(query.iter_mut().count(), 4);
}

fn test_query13(query: Query<(), (Added<A>, With<A>)>, query_check: Query<(), Added<A>>) {
    test_query13.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query14(query: Query<(), (Changed<A>, With<A>)>, query_check: Query<(), Changed<A>>) {
    test_query14.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query15(query: (Query<&A, With<A>>,), query_check: Query<&A>) {
    test_query15.system();
    assert_eq!(query.0.iter().count(), query_check.iter().count());
}

fn test_query16(query: (((((((((Query<&A, With<A>>,),),),),),),),),), query_check: Query<&A>) {
    test_query16.system();
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
    test_query17.system();
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
    test_query18.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query19(
    query: Query<
        ((), ((((), (((((&A,), ()),),), (), ((),))),),)),
        ((), ((((), (((((), ()),),), (), (With<A>,))),),)),
    >,
    query_check: Query<&A>,
) {
    test_query19.system();
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
    test_query20.system();
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
    test_query21::<B>.system();
    assert_eq!(query.iter().count(), query_check.iter().count());
}

fn test_query22(mut query: Query<Option<(&A, With<A>)>>, query_check: Query<&A>) {
    test_query22.system();
    assert_eq!(
        query.iter_mut().filter(|option| option.is_some()).count(),
        query_check.iter().count()
    );
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
        Self::system_param_test.system();

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
        Self::test_trait1_query1.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query2(
        query: Query<&Self::TestType, With<Self::TestType>>,
        query_check: Query<&Self::TestType>,
    ) {
        Self::test_trait1_query2.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query3(query: Query<&Self, With<Self::TestType>>, query_check: Query<&Self>) {
        Self::test_trait1_query3.system();
        let _ = query;
        let _ = query_check;
    }
    fn test_trait1_query4(
        query: Query<&Self::TestType, With<Self>>,
        query_check: Query<&Self::TestType>,
    ) {
        Self::test_trait1_query4.system();
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
        Self::test_trait1_query1.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query2(
        query: Query<&Self::TestType, With<Self::TestType>>,
        query_check: Query<&Self::TestType>,
    ) {
        Self::test_trait1_query2.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query3(query: Query<&Self, With<Self::TestType>>, query_check: Query<&Self>) {
        Self::test_trait1_query3.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query4(
        query: Query<&Self::TestType, With<Self>>,
        query_check: Query<&Self::TestType>,
    ) {
        Self::test_trait1_query4.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
}

impl TestTrait1 for C {
    type TestType = C;

    fn test_trait1_query1(query: Query<&Self, With<Self>>, query_check: Query<&Self>) {
        Self::test_trait1_query1.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query2(
        query: Query<&Self::TestType, With<Self::TestType>>,
        query_check: Query<&Self::TestType>,
    ) {
        Self::test_trait1_query2.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query3(query: Query<&Self, With<Self::TestType>>, query_check: Query<&Self>) {
        Self::test_trait1_query3.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
    }
    fn test_trait1_query4(
        query: Query<&Self::TestType, With<Self>>,
        query_check: Query<&Self::TestType>,
    ) {
        Self::test_trait1_query4.system();
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
        Self::test_trait2_query1.system();
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
        Self::test_trait2_query1.system();
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
        Self::test_trait2_query1.system();
        assert_eq!(query.iter().count(), query_check.iter().count());
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
        .add_system(test_query9)
        .add_system(test_query10)
        .add_system(test_query11)
        .add_system(test_query12)
        .add_system(test_query13)
        .add_system(test_query14)
        .add_system(test_query15)
        .add_system(test_query16)
        .add_system(test_query17)
        .add_system(test_query18)
        .add_system(test_query19)
        .add_system(test_query20)
        .add_system(test_query21::<A>)
        .add_system(test_query22)
        .add_system(SystemParamTest::system_param_test)
        .add_system(A::test_trait1_query1)
        .add_system(A::test_trait1_query2)
        .add_system(A::test_trait1_query3)
        .add_system(A::test_trait1_query4)
        .add_system(B::test_trait1_query1)
        .add_system(B::test_trait1_query2)
        .add_system(B::test_trait1_query3)
        .add_system(B::test_trait1_query4)
        .add_system(C::test_trait1_query1)
        .add_system(C::test_trait1_query2)
        .add_system(C::test_trait1_query3)
        .add_system(C::test_trait1_query4)
        .add_system(A::test_trait2_query1)
        .add_system(D::<C>::test_trait2_query1)
        .run();
}

fn setup(mut commands: Commands) {
    let d: D<A> = D::default();

    commands.spawn_bundle((A,));
    commands.spawn_bundle((B,));
    commands.spawn_bundle((C,));
    commands.spawn_bundle((d,));
    commands.spawn_bundle((A, B));
    commands.spawn_bundle((B, C));
    commands.spawn_bundle((C, d));
    commands.spawn_bundle((A, C));
    commands.spawn_bundle((B, d));
    commands.spawn_bundle((A, d));
    commands.spawn_bundle((A, B, C));
    commands.spawn_bundle((B, C, d));
    commands.spawn_bundle((A, C, d));
    commands.spawn_bundle((A, B, d));
    commands.spawn_bundle((A, B, C, d));
}
