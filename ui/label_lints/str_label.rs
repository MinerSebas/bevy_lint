use bevy::{
    app::prelude::*,
    ecs::{prelude::*, schedule::ShouldRun, system::BoxedSystem},
};

fn some_system() {}

fn main() {
    App::new()
        // SystemLabel
        .add_system(some_system.label("SomeLabel"))
        .add_system(some_system.before("SomeLabel"))
        .add_system(some_system.after("SomeLabel"))
        .add_system(some_system.exclusive_system().label("SomeLabel"))
        .add_system(some_system.exclusive_system().before("SomeLabel"))
        .add_system(some_system.exclusive_system().after("SomeLabel"))
        .add_system((Box::new(some_system.system()) as BoxedSystem).label("SomeLabel"))
        .add_system((Box::new(some_system.system()) as BoxedSystem).before("SomeLabel"))
        .add_system((Box::new(some_system.system()) as BoxedSystem).after("SomeLabel"))
        .add_system_set(SystemSet::new().with_system(some_system).label("SomeLabel"))
        .add_system_set(
            SystemSet::new()
                .with_system(some_system)
                .before("SomeLabel"),
        )
        .add_system_set(SystemSet::new().with_system(some_system).after("SomeLabel"))
        // AmbiguitySetLabel
        .add_system(some_system.in_ambiguity_set("AmbLabel"))
        .add_system(some_system.exclusive_system().in_ambiguity_set("AmbLabel"))
        .add_system((Box::new(some_system.system()) as BoxedSystem).in_ambiguity_set("AmbLabel"))
        .add_system_set(
            SystemSet::new()
                .with_system(some_system)
                .in_ambiguity_set("AmbLabel"),
        )
        // RunCriteriaLabel
        .add_system(some_system.with_run_criteria((|| ShouldRun::Yes).label("RunLabel")))
        .add_system(some_system.with_run_criteria((|| ShouldRun::Yes).before("RunLabel")))
        .add_system(some_system.with_run_criteria((|| ShouldRun::Yes).after("RunLabel")))
        .add_system(
            some_system
                .with_run_criteria((|| ShouldRun::Yes).label_discard_if_duplicate("RunLabel")),
        )
        .add_system(
            some_system.with_run_criteria(
                (Box::new((|| ShouldRun::Yes).system()) as BoxedSystem<(), ShouldRun>)
                    .label("RunLabel"),
            ),
        )
        .add_system(
            some_system.with_run_criteria(
                (Box::new((|| ShouldRun::Yes).system()) as BoxedSystem<(), ShouldRun>)
                    .before("RunLabel"),
            ),
        )
        .add_system(
            some_system.with_run_criteria(
                (Box::new((|| ShouldRun::Yes).system()) as BoxedSystem<(), ShouldRun>)
                    .after("RunLabel"),
            ),
        )
        .add_system(
            some_system.with_run_criteria(
                (Box::new((|| ShouldRun::Yes).system()) as BoxedSystem<(), ShouldRun>)
                    .label_discard_if_duplicate("RunLabel"),
            ),
        );
}
