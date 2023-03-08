use std::collections::HashSet;

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::domain::common::yaml_conversion::YamlConversion;

use super::{event::EventTrigger, job::Job};

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

    // Not quite an implementation for YamlConversion
    // Curses be upon thee rust_yaml for not having a Yaml variant to represent
    // a top level file!
    pub fn to_yaml(&self) -> Vec<Yaml> {
        let mut file = Vec::new();

        let triggers = self
            .triggers
            .iter()
            .map(|trigger| trigger.to_yaml())
            .collect();

        let mut map = LinkedHashMap::new();
        map.insert(Yaml::String(String::from("on")), Yaml::Array(triggers));

        file.push(Yaml::Hash(map));

        file
    }
}
