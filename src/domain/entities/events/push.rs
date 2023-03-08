use std::collections::HashMap;

use yaml_rust::Yaml;

use crate::domain::{
    common::yaml_conversion::{YamlConversion, YamlKey},
    entities::{
        event::Event,
        filter::{FilterExpression, Filterable},
    },
};

#[derive(Default)]
pub struct PushEvent {
    pub filters: HashMap<PushEventFilter, FilterExpression>,
}

impl YamlKey for PushEvent {
    fn get_identifier(&self) -> String {
        String::from("push")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum PushEventFilter {
    Branches,
    Tags,
    BranchesIgnore,
    TagsIgnore,
}

impl YamlKey for PushEventFilter {
    fn get_identifier(&self) -> String {
        String::from(match self {
            PushEventFilter::Branches => "branches",
            PushEventFilter::Tags => "tags",
            PushEventFilter::BranchesIgnore => "branches-ignore",
            PushEventFilter::TagsIgnore => "tags-ignore",
        })
    }
}

impl Filterable for PushEvent {
    type FilterType = PushEventFilter;
}

impl Event for PushEvent {
    fn filter_yaml(&self) -> Option<Yaml> {
        if self.filters.is_empty() {
            None
        } else {
            Some(self.filters.to_yaml())
        }
    }

    fn type_yaml(&self) -> Option<Yaml> {
        None
    }
}
