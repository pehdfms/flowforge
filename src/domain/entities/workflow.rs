use std::collections::HashSet;

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::domain::common::yaml_conversion::YamlConversion;

use super::{
    event::{Event, EventTrigger},
    job::Job,
};

#[derive(Default)]
pub struct Workflow {
    name: String,
    triggers: HashSet<EventTrigger>,
    jobs: Vec<Job>,
}

impl Workflow {
    pub fn new(name: String, triggered_by: HashSet<EventTrigger>, jobs: Vec<Job>) -> Self {
        Self {
            name,
            triggers: triggered_by,
            jobs,
        }
    }

    pub fn get_triggers(&self) -> &HashSet<EventTrigger> {
        &self.triggers
    }

    pub fn add_trigger(mut self, trigger: impl Into<EventTrigger>) -> Self {
        self.triggers.insert(trigger.into());
        self
    }
}

impl YamlConversion for Workflow {
    fn to_yaml(&self) -> String {
        let triggers = self.triggers.iter().map(|trigger| trigger.to_yaml());

        let map = LinkedHashMap::new();
        map.insert(Yaml::String("on"), Yaml::Array(self.triggers.));
        todo!()
    }
}
