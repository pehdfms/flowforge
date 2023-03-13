use std::mem;

use serde_yaml::{Mapping, Value};

use crate::domain::common::yaml_conversion::YamlConversion;

use super::step::Step;

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

pub struct JobBuilder {
    job: Job,
}

impl JobBuilder {
    pub fn new(name: impl Into<String>, runs_on: impl Into<String>) -> Self {
        Self {
            job: Job::new(name.into(), Vec::default(), runs_on.into(), Vec::default()),
        }
    }

    pub fn build(&mut self) -> Job {
        mem::take(&mut self.job)
    }

    pub fn needs(&mut self, jobs: Vec<String>) -> &mut Self {
        self.job.needs = jobs;
        self
    }

    pub fn needs_checked<'a>(&mut self, jobs: &[Job]) -> &mut Self {
        self.job.needs = jobs.iter().map(|job| job.name.to_string()).collect();
        self
    }

    pub fn runs_on(&mut self, env: String) -> &mut Self {
        self.job.runs_on = env;
        self
    }

    pub fn get_steps(&self) -> &[Step] {
        &self.job.steps
    }

    pub fn steps(&mut self, steps: Vec<Step>) -> &mut Self {
        self.job.steps = steps;
        self
    }

    pub fn steps_from<T>(&mut self, steps: impl Into<Vec<T>>) -> &mut Self
    where
        T: Into<Step>,
    {
        self.job.steps = steps.into().into_iter().map(Into::into).collect();
        self
    }

    pub fn add_step(&mut self, step: impl Into<Step>) -> &mut Self {
        self.job.steps.push(step.into());
        self
    }
}
