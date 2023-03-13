use serde_yaml::{Mapping, Value};

use super::{common::yaml_conversion::YamlConversion, step::Step};

#[derive(Default)]
pub struct Job {
    name: String,
    needs: Vec<String>, // TODO improve type safety
    runs_on: String,
    steps: Vec<Step>, // TODO improve representation
}

impl Job {
    pub fn new(name: String, needs: Vec<String>, runs_on: String, steps: Vec<Step>) -> Self {
        Self {
            name,
            needs,
            runs_on,
            steps,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl YamlConversion for Job {
    fn to_yaml(&self) -> Value {
        let mut map = Mapping::new();

        if !self.needs.is_empty() {
            map.insert(Value::from("needs"), Value::from(self.needs.clone()));
        }

        map.insert(Value::from("runs-on"), Value::from(self.runs_on.clone()));
        map.extend(Mapping::from_iter(vec![(
            Value::from("steps"),
            Value::from(
                self.steps
                    .iter()
                    .map(|step| step.to_yaml())
                    .collect::<Vec<_>>(),
            ),
        )]));

        Value::from(Mapping::from_iter(vec![(
            Value::from(self.name.clone()),
            Value::from(map),
        )]))
    }
}

pub mod builder;
