use std::{collections::HashSet, convert::Infallible};

use serde_yaml::{Mapping, Value};
use thiserror::Error;

use super::{event::EventTrigger, job::Job};

#[derive(Default)]
pub struct Workflow {
    name: String,
    triggers: HashSet<EventTrigger>,
    jobs: Vec<Job>,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorkflowBuildError {
    #[error("Workflow lacks Triggers")]
    NoTriggers,
    #[error("Workflow lacks Jobs")]
    NoJobs,
}

impl Workflow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            triggers: HashSet::default(),
            jobs: Vec::default(),
        }
    }

    pub fn jobs(&mut self, jobs: Vec<Job>) -> &mut Self {
        self.jobs = jobs;
        self
    }

    pub fn jobs_from(&mut self, jobs: impl Into<Vec<Job>>) -> &mut Self {
        self.jobs = jobs.into().into_iter().map(Into::into).collect();
        self
    }

    pub fn add_job(&mut self, job: Job) -> &mut Self {
        self.jobs.push(job);
        self
    }

    pub fn get_triggers(&self) -> &HashSet<EventTrigger> {
        &self.triggers
    }

    pub fn on(&mut self, triggers: HashSet<EventTrigger>) -> &mut Self {
        self.triggers = triggers;
        self
    }

    pub fn triggers_from<T>(&mut self, triggers: impl Into<Vec<T>>) -> &mut Self
    where
        T: Into<EventTrigger>,
    {
        self.triggers = triggers.into().into_iter().map(Into::into).collect();
        self
    }

    pub fn add_trigger(&mut self, trigger: impl Into<EventTrigger>) -> &mut Self {
        self.triggers.insert(trigger.into());
        self
    }

    pub fn build(&self) -> Result<String, Vec<WorkflowBuildError>> {
        let mut errors = Vec::new();

        if self.triggers.is_empty() {
            errors.push(WorkflowBuildError::NoTriggers);
        }

        if self.jobs.is_empty() {
            errors.push(WorkflowBuildError::NoJobs);
        }

        if errors.is_empty() {
            Ok(String::from("new"))
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_yaml::{Mapping, Value};

    use crate::domain::entities::{
        event::EventTrigger, events::push::PushEvent, job::JobBuilder, step::RunStep,
    };

    use super::{Workflow, WorkflowBuildError};

    #[test]
    fn test_invalid_build() {
        // TODO add more cases
        let results = [
            (
                "no_triggers_or_jobs",
                Workflow::new("no_triggers_or_jobs").build(),
                vec![WorkflowBuildError::NoTriggers, WorkflowBuildError::NoJobs],
            ),
            (
                "empty_triggers_no_jobs",
                Workflow::new("empty_triggers_no_jobs")
                    .triggers_from(Vec::<EventTrigger>::new())
                    .build(),
                vec![WorkflowBuildError::NoTriggers, WorkflowBuildError::NoJobs],
            ),
        ];

        for (workflow, result, expected_errors) in results {
            let errors = result.expect_err(format!("should fail workflow build when having no triggers or jobs, expected workflow with name '{workflow}' to fail but found no errors").as_str());
            assert_eq!(
                errors, expected_errors,
                "errors returned from workflow named '{workflow}' did not match expected errors"
            );
        }
    }

    #[test]
    fn test_simple_build() {
        let workflow = Workflow::new("Simple")
            .add_trigger(PushEvent::new())
            .add_job(
                JobBuilder::new("build", "ubuntu-latest")
                    .add_step(RunStep::new("echo \"Hello, world!\"").with_name("Hello World"))
                    .build(),
            )
            .build()
            .unwrap();

        // TODO github actions isn't valid yaml so we'll need to fork rust_yaml
        // to allow the following
        // I'll probably do this by just serializing null as empty
        assert_eq!(
            workflow,
            "
            name: Simple

            on:
              push:

            jobs:
              build:
                runs-on: ubuntu-latest
                steps:
                  - name: Hello World
                    run: echo \"Hello, world!\"
            "
        )
    }
}
