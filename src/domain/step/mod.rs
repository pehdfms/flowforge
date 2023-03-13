use enum_dispatch::enum_dispatch;
use serde_yaml::Value;

use self::{action_step::ActionStep, run_step::RunStep};

use super::common::yaml_conversion::YamlConversion;

#[enum_dispatch]
trait JobStep {}

#[enum_dispatch(JobStep, YamlConversion)]
pub enum Step {
    ActionStep,
    RunStep,
}

pub mod action_step;
pub mod builder;
pub mod run_step;
