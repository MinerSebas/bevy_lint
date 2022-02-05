use bevy::{
    app::prelude::*,
    ecs::{prelude::*, schedule::ShouldRun, system::BoxedSystem},
};

fn some_system() {}

fn main() {
    let mut app = App::new();

    // SystemLabel
    app.add_system(some_system.label("SomeLabel"))
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
        )
        /* TODO: "pipe" is a expr of kind "Call" and not "MethodCall"
        .add_system(some_system.with_run_criteria(RunCriteria::pipe(
            "RunLabel",
            (|input: In<ShouldRun>| input.0).system(),
        )))*/
        // StageLabel
        .add_stage("StageLabel", SystemStage::parallel())
        .add_stage_before("StageLabel", "StageLabel_2", SystemStage::parallel())
        .add_stage_after("StageLabel", "StageLabel_3", SystemStage::parallel())
        .add_startup_stage("StageLabel", SystemStage::parallel())
        .add_startup_stage_before("StageLabel", "StageLabel_2", SystemStage::parallel())
        .add_startup_stage_after("StageLabel", "StageLabel_3", SystemStage::parallel())
        .stage("StageLabel", |stage: &mut SystemStage| stage)
        .add_system_to_stage("StageLabel", some_system)
        .add_system_set_to_stage("StageLabel", SystemSet::new())
        .add_startup_system_to_stage("StageLabel", some_system)
        .add_startup_system_set_to_stage("StageLabel", SystemSet::new())
        // AppLabel
        .add_sub_app("AppLabel", App::new(), |_, _| {});

    app.sub_app_mut("AppLabel");
    app.sub_app("AppLabel");
    // TODO: Dont work as their return type is Result<App, _>, not App
    //let _ = app.get_sub_app_mut("AppLabel");
    //let _ = app.get_sub_app("AppLabel");
}
