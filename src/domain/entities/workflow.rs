use std::collections::HashSet;

use crate::domain::common::yaml_conversion::YamlConversion;

use super::{event::EventTrigger, job::Job};

#[derive(Default)]
pub struct Workflow {
    triggers: HashSet<EventTrigger>,
    jobs: Vec<Job>,
}

impl Workflow {
    pub fn new(triggered_by: HashSet<EventTrigger>, jobs: Vec<Job>) -> Self {
        Self {
            triggers: triggered_by,
            jobs,
        }
    }

    pub fn add_trigger(&mut self, trigger: impl Into<EventTrigger>) {
        self.triggers.insert(trigger.into());
    }

    pub fn contains_trigger<'a>(&self, trigger: impl Into<&'a EventTrigger>) -> bool {
        self.triggers.contains(trigger.into())
    }
}

impl YamlConversion for Workflow {
    fn to_yaml(&self) -> String {
        todo!()
    }
}
