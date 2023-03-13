use serde_yaml::Value;

use crate::domain::common::yaml_conversion::YamlConversion;

pub struct ActionStep {
    name: Option<String>,
    uses: String,
    with: Option<String>,
}

impl ActionStep {
    pub fn new(name: Option<String>, uses: String, with: Option<String>) -> Self {
        Self { name, uses, with }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn set_name(&mut self, name: Option<String>) -> &mut Self {
        self.name = name;
        self
    }
}

impl YamlConversion for ActionStep {
    fn to_yaml(&self) -> Value {
        todo!()
    }
}
