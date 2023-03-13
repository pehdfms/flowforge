use std::mem;

use crate::domain::step::Step;

use super::Job;

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

    pub fn needs<T>(&mut self, jobs: impl Into<Vec<T>>) -> &mut Self
    where
        T: Into<String>,
    {
        self.job.needs = jobs.into().into_iter().map(|s| s.into()).collect();
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
