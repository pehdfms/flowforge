use std::mem;

use super::{action_step::ActionStep, run_step::RunStep};

pub struct StepBuilder<S: StepBuildState> {
    building: S,
}

impl StepBuilder<BuildingUnknownStep> {
    pub fn new() -> StepBuilder<BuildingUnknownStep> {
        StepBuilder {
            building: BuildingUnknownStep { name: None },
        }
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.building.name = Some(name.into());
        self
    }

    pub fn using_action(&mut self, action: impl Into<String>) -> StepBuilder<BuildingActionStep> {
        StepBuilder {
            building: BuildingActionStep {
                action_step: ActionStep::new(
                    mem::take(&mut self.building.name),
                    action.into(),
                    None,
                ),
            },
        }
    }

    pub fn with_command(&mut self, command: impl Into<String>) -> StepBuilder<BuildingRunStep> {
        StepBuilder {
            building: BuildingRunStep {
                run_step: RunStep::new(mem::take(&mut self.building.name), command.into()),
            },
        }
    }
}

impl StepBuilder<BuildingActionStep> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.building.action_step.set_name(Some(name.into()));
        self
    }

    pub fn build(self) -> ActionStep {
        self.building.action_step
    }
}

impl StepBuilder<BuildingRunStep> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.building.run_step.set_name(Some(name.into()));
        self
    }

    pub fn build(self) -> RunStep {
        self.building.run_step
    }
}

pub struct BuildingUnknownStep {
    name: Option<String>,
}

pub struct BuildingActionStep {
    action_step: ActionStep,
}

pub struct BuildingRunStep {
    run_step: RunStep,
}

pub trait StepBuildState: private::Sealed {}

impl StepBuildState for BuildingUnknownStep {}
impl StepBuildState for BuildingActionStep {}
impl StepBuildState for BuildingRunStep {}

mod private {
    use super::{BuildingActionStep, BuildingRunStep, BuildingUnknownStep};

    pub trait Sealed {}

    impl Sealed for BuildingUnknownStep {}
    impl Sealed for BuildingActionStep {}
    impl Sealed for BuildingRunStep {}
}
