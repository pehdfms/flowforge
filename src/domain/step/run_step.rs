use serde_yaml::{Mapping, Value};

use crate::domain::common::yaml_conversion::YamlConversion;

pub struct RunStep {
    name: Option<String>,
    command: String,
}

impl RunStep {
    pub fn new(name: Option<String>, command: String) -> Self {
        Self { name, command }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: Option<String>) -> &mut Self {
        self.name = name;
        self
    }
}

impl YamlConversion for RunStep {
    fn to_yaml(&self) -> Value {
        let mut map = Mapping::new();

        if let Some(name) = &self.name {
            map.insert(Value::from("name"), Value::from(name.to_string()));
        }

        map.insert(Value::from("run"), Value::from(self.command.to_string()));
        Value::from(map)
    }
}
