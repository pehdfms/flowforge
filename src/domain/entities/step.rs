use enum_dispatch::enum_dispatch;
use serde_yaml::{Mapping, Value};

use crate::domain::common::yaml_conversion::YamlConversion;

// TODO make a builder for these
pub struct ActionStep {
    name: Option<String>,
    uses: String,
    with: Option<String>,
}

impl ActionStep {
    pub fn new(uses: String) -> Self {
        Self {
            name: None,
            uses,
            with: None,
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
}

impl YamlConversion for ActionStep {
    fn to_yaml(&self) -> serde_yaml::Value {
        todo!()
    }
}

pub struct RunStep {
    name: Option<String>,
    command: String,
}

impl RunStep {
    pub fn new(command: impl Into<String>) -> Self {
        Self {
            name: None,
            command: command.into(),
        }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn with_name(self, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            ..self
        }
    }
}

impl YamlConversion for RunStep {
    fn to_yaml(&self) -> serde_yaml::Value {
        let mut map = Mapping::new();

        if let Some(name) = &self.name {
            map.insert(Value::from("name"), Value::from(name.to_string()));
        }

        map.insert(Value::from("run"), Value::from(self.command.to_string()));
        Value::from(map)
    }
}

#[enum_dispatch]
trait JobStep {}

#[enum_dispatch(JobStep, YamlConversion)]
pub enum Step {
    ActionStep,
    RunStep,
}
