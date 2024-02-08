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

fn test_query1(query: Query<Option<&A>, With<A>>) {
    assert_is_system(test_query1);

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query2(mut query: Query<Option<&mut A>, With<A>>) {
    assert_is_system(test_query2);

    for option in query.iter_mut() {
        assert!(option.is_some());
    }
}

fn test_query3(query: Query<Option<&A>, (With<A>, With<B>)>) {
    assert_is_system(test_query3);

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query4(mut query: Query<Option<&mut A>, (With<A>, With<B>)>) {
    assert_is_system(test_query4);

    for option in query.iter_mut() {
        assert!(option.is_some());
    }
}

fn test_query5(query: Query<Option<&B>, (With<A>, With<B>)>) {
    assert_is_system(test_query5);

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query6(mut query: Query<Option<&mut B>, (With<A>, With<B>)>) {
    assert_is_system(test_query6);

    for option in query.iter_mut() {
        assert!(option.is_some());
    }
}

fn test_query7(query: Query<(Option<&A>, &B), With<A>>) {
    assert_is_system(test_query7);

    for (option, _) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query8(query: Query<(&A, Option<&B>), With<B>>) {
    assert_is_system(test_query8);

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query9(query: Query<Option<&A>, Without<A>>) {
    assert_is_system(test_query9);

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query10(mut query: Query<Option<&mut A>, Without<A>>) {
    assert_is_system(test_query10);

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query11(query: Query<Option<&A>, (Without<A>, Without<B>)>) {
    assert_is_system(test_query11);

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query12(mut query: Query<Option<&mut A>, (Without<A>, Without<B>)>) {
    assert_is_system(test_query12);

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query13(query: Query<Option<&B>, (Without<A>, Without<B>)>) {
    assert_is_system(test_query13);

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query14(mut query: Query<Option<&mut B>, (Without<A>, Without<B>)>) {
    assert_is_system(test_query14);

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query15(query: Query<(Option<&A>, &B), Added<A>>) {
    assert_is_system(test_query15);

    for (option, _) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query16(query: Query<(&A, Option<&B>), Changed<B>>) {
    assert_is_system(test_query16);

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query17(query: Query<(&A, &B, Option<(&A, &B)>)>) {
    assert_is_system(test_query17);

    for (_, _, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query18(query: Query<(&A, Option<(&A, &B)>), With<B>>) {
    assert_is_system(test_query18);

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query19(query: Query<(&A, Option<(&A, &B)>), Added<B>>) {
    assert_is_system(test_query19);

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query20(query: Query<(&A, Option<(&A, &B)>), Changed<B>>) {
    assert_is_system(test_query20);

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query21(query: Query<(&A, Option<(&A, &B)>), Without<B>>) {
    assert_is_system(test_query21);

    for (_, option) in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query22(mut query: Query<(&A, &B, Option<(&A, Without<B>)>)>) {
    assert_is_system(test_query22);

    for (_, _, option) in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query23(query: Query<(&A, Option<(&B, Option<&A>)>)>) {
    assert_is_system(test_query23);

    for (_, option) in query.iter().filter(|result| result.1.is_some()) {
        assert!(option.unwrap().1.is_some());
    }
}

fn test_query24(query: Query<Option<(&A, Option<&B>)>, Without<A>>) {
    assert_is_system(test_query24);

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query25(query: Query<Option<(&A, Option<&B>)>, (With<A>, Without<B>)>) {
    assert_is_system(test_query25);

    for option in query.iter() {
        assert!(option.is_some());
        assert!(option.unwrap().1.is_none());
    }
}

fn test_query26(query: Query<(&A, &B, Option<(&A, Option<&B>)>)>) {
    assert_is_system(test_query26);

    for (_, _, option) in query.iter() {
        assert!(option.is_some());
        assert!(option.unwrap().1.is_some());
    }
}

fn test_query27(mut query: Query<Option<(&A, Without<A>)>>) {
    assert_is_system(test_query27);

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query28(query: Query<Option<Option<&A>>, Without<A>>) {
    assert_is_system(test_query28);

    for option in query.iter() {
        assert!(option.is_some());
        assert!(option.unwrap().is_none());
    }
}

fn test_query29(query: Query<Option<&A>, Added<A>>) {
    assert_is_system(test_query29);

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query30(query: Query<Option<&A>, Changed<A>>) {
    assert_is_system(test_query30);

    for option in query.iter() {
        assert!(option.is_some());
    }
}

// This may not trigger the Lint
fn negativ_test_query1(query: Query<Option<&A>, Or<(With<A>, With<B>)>>) {
    assert_is_system(negativ_test_query1);

    let mut is_some = false;
    let mut is_none = false;

    for option in query.iter() {
        match option {
            Some(_) => is_some = true,
            None => is_none = true,
        }
    }

    assert!(is_some);
    assert!(is_none);
}
// This may not trigger the Lint
fn negativ_test_query2(query: Query<Option<&A>, Or<(Without<A>, Without<B>)>>) {
    assert_is_system(negativ_test_query2);

    let mut is_some = false;
    let mut is_none = false;

    for option in query.iter() {
        match option {
            Some(_) => is_some = true,
            None => is_none = true,
        }
    }

    assert!(is_some);
    assert!(is_none);
}
// This may not trigger the Lint (but should trigger the empty-query lint)
fn negativ_test_query3(query: Query<Option<&A>, (With<A>, Without<A>)>) {
    assert_is_system(negativ_test_query3);

    assert_eq!(query.iter().count(), 0);
}
// This may not trigger the Lint
fn negativ_test_query4(query: Query<Option<&A>>) {
    assert_is_system(negativ_test_query4);

    let mut is_some = false;
    let mut is_none = false;

    for option in query.iter() {
        match option {
            Some(_) => is_some = true,
            None => is_none = true,
        }
    }

    assert!(is_some);
    assert!(is_none);
}

#[derive(SystemParam)]
struct SystemParamTest<'w, 's> {
    query1: Query<'w, 's, Option<&'static A>, With<A>>,
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
                            (Option<&'static A>, Option<&'static B>),
                            (Changed<A>, With<B>),
                        >,
                    ),
                    (),
                ),),),
                (),
                (
                    Query<
                        'w,
                        's,
                        (Option<&'static A>, Option<&'static C>),
                        (Or<(With<A>, With<B>)>, With<C>),
                    >,
                    (),
                ),
            ),
        ),),),
    ),
}

impl<'w, 's> SystemParamTest<'w, 's> {
    fn system_param_test(system_param: SystemParamTest) {
        assert_is_system(Self::system_param_test);

        for option in system_param.query1.iter() {
            assert!(option.is_some());
        }
        for (option_a, option_b) in system_param.query2.1 .0 .0 .1 .0 .0 .0 .0 .0.iter() {
            assert!(option_a.is_some());
            assert!(option_b.is_some());
        }
        for (_, option) in system_param.query2.1 .0 .0 .1 .2 .0.iter() {
            assert!(option.is_some());
        }
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
        .add_systems(Update, test_query21)
        .add_systems(Update, test_query22)
        .add_systems(Update, test_query23)
        .add_systems(Update, test_query24)
        .add_systems(Update, test_query25)
        .add_systems(Update, test_query26)
        .add_systems(Update, test_query27)
        .add_systems(Update, test_query28)
        .add_systems(Update, test_query29)
        .add_systems(Update, test_query30)
        .add_systems(Update, negativ_test_query1)
        .add_systems(Update, negativ_test_query2)
        .add_systems(Update, negativ_test_query3)
        .add_systems(Update, negativ_test_query4)
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
