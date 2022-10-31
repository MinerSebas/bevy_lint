#![allow(dead_code)]
use bevy::{app::AppLabel, ecs::prelude::*};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    StageLabel,
    SystemLabel,
    RunCriteriaLabel,
    AmbiguitySetLabel,
    AppLabel,
)]
struct AllLables;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    StageLabel,
    SystemLabel,
    RunCriteriaLabel,
    AmbiguitySetLabel,
)]
enum FourLables {
    Variant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, StageLabel, SystemLabel, RunCriteriaLabel)]
enum ThreeLabels {
    Variant1,
    Variant2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, StageLabel, SystemLabel)]
struct TwoLabels;

// Should not trigger Lint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, StageLabel)]
struct StageLabelStruct;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemLabel)]
struct SystemLabelStruct;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, RunCriteriaLabel)]
struct RunCriteriaLabelStruct;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AmbiguitySetLabel)]
struct AmbiguitySetLabelStruct;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, AppLabel)]
struct AppLabelStruct;

fn main() {
    // Nothing to test here at runtime.
}
