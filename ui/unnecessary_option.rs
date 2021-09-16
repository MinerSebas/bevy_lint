#![allow(dead_code)]
#![allow(clippy::type_complexity)]
use bevy::{
    app::App,
    ecs::{prelude::*, system::SystemParam},
};

#[derive(Debug, Component)]
struct A;
#[derive(Debug, Component)]
struct B;
#[derive(Debug, Component)]
struct C;

fn test_query1(query: Query<Option<&A>, With<A>>) {
    test_query1.system();

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query2(mut query: Query<Option<&mut A>, With<A>>) {
    test_query2.system();

    for option in query.iter_mut() {
        assert!(option.is_some());
    }
}

fn test_query3(query: Query<Option<&A>, (With<A>, With<B>)>) {
    test_query3.system();

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query4(mut query: Query<Option<&mut A>, (With<A>, With<B>)>) {
    test_query4.system();

    for option in query.iter_mut() {
        assert!(option.is_some());
    }
}

fn test_query5(query: Query<Option<&B>, (With<A>, With<B>)>) {
    test_query5.system();

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query6(mut query: Query<Option<&mut B>, (With<A>, With<B>)>) {
    test_query6.system();

    for option in query.iter_mut() {
        assert!(option.is_some());
    }
}

fn test_query7(query: Query<(Option<&A>, &B), With<A>>) {
    test_query7.system();

    for (option, _) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query8(query: Query<(&A, Option<&B>), With<B>>) {
    test_query8.system();

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query9(query: Query<Option<&A>, Without<A>>) {
    test_query9.system();

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query10(mut query: Query<Option<&mut A>, Without<A>>) {
    test_query10.system();

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query11(query: Query<Option<&A>, (Without<A>, Without<B>)>) {
    test_query11.system();

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query12(mut query: Query<Option<&mut A>, (Without<A>, Without<B>)>) {
    test_query12.system();

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query13(query: Query<Option<&B>, (Without<A>, Without<B>)>) {
    test_query13.system();

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query14(mut query: Query<Option<&mut B>, (Without<A>, Without<B>)>) {
    test_query14.system();

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query15(query: Query<(Option<&A>, &B), Added<A>>) {
    test_query15.system();

    for (option, _) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query16(query: Query<(&A, Option<&B>), Changed<B>>) {
    test_query16.system();

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query17(query: Query<(&A, &B, Option<(&A, &B)>)>) {
    test_query17.system();

    for (_, _, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query18(query: Query<(&A, Option<(&A, &B)>), With<B>>) {
    test_query18.system();

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query19(query: Query<(&A, Option<(&A, &B)>), Added<B>>) {
    test_query19.system();

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query20(query: Query<(&A, Option<(&A, &B)>), Changed<B>>) {
    test_query20.system();

    for (_, option) in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query21(query: Query<(&A, Option<(&A, &B)>), Without<B>>) {
    test_query21.system();

    for (_, option) in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query22(mut query: Query<(&A, &B, Option<(&A, Without<B>)>)>) {
    test_query22.system();

    for (_, _, option) in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query23(query: Query<(&A, Option<(&B, Option<&A>)>)>) {
    test_query23.system();

    for (_, option) in query.iter().filter(|result| result.1.is_some()) {
        assert!(option.unwrap().1.is_some());
    }
}

fn test_query24(query: Query<Option<(&A, Option<&B>)>, Without<A>>) {
    test_query24.system();

    for option in query.iter() {
        assert!(option.is_none());
    }
}

fn test_query25(query: Query<Option<(&A, Option<&B>)>, (With<A>, Without<B>)>) {
    test_query25.system();

    for option in query.iter() {
        assert!(option.is_some());
        assert!(option.unwrap().1.is_none());
    }
}

fn test_query26(query: Query<(&A, &B, Option<(&A, Option<&B>)>)>) {
    test_query26.system();

    for (_, _, option) in query.iter() {
        assert!(option.is_some());
        assert!(option.unwrap().1.is_some());
    }
}

fn test_query27(mut query: Query<Option<(&A, Without<A>)>>) {
    test_query27.system();

    for option in query.iter_mut() {
        assert!(option.is_none());
    }
}

fn test_query28(query: Query<Option<Option<&A>>, Without<A>>) {
    test_query28.system();

    for option in query.iter() {
        assert!(option.is_some());
        assert!(option.unwrap().is_none());
    }
}

fn test_query29(query: Query<Option<&A>, Added<A>>) {
    test_query29.system();

    for option in query.iter() {
        assert!(option.is_some());
    }
}

fn test_query30(query: Query<Option<&A>, Changed<A>>) {
    test_query30.system();

    for option in query.iter() {
        assert!(option.is_some());
    }
}

// This may not trigger the Lint
fn negativ_test_query1(query: Query<Option<&A>, Or<(With<A>, With<B>)>>) {
    negativ_test_query1.system();

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
    negativ_test_query2.system();

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
    negativ_test_query3.system();

    assert_eq!(query.iter().count(), 0);
}
// This may not trigger the Lint
fn negativ_test_query4(query: Query<Option<&A>>) {
    negativ_test_query4.system();

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
                            (Option<&'static mut A>, Option<&'static B>),
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
                        (Option<&'static mut A>, Option<&'static C>),
                        (Or<(With<A>, With<B>)>, With<C>),
                    >,
                    (),
                ),
            ),
        ),),),
    ),
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
        .add_system(test_query21)
        .add_system(test_query22)
        .add_system(test_query23)
        .add_system(test_query24)
        .add_system(test_query25)
        .add_system(test_query26)
        .add_system(test_query27)
        .add_system(test_query28)
        .add_system(test_query29)
        .add_system(test_query30)
        .add_system(negativ_test_query1)
        .add_system(negativ_test_query2)
        .add_system(negativ_test_query3)
        .add_system(negativ_test_query4)
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
