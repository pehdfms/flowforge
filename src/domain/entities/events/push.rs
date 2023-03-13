use std::collections::HashMap;

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

impl PushEvent {
    pub fn new() -> Self {
        Self::default()
    }
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
            Self::Branches => "branches",
            Self::Tags => "tags",
            Self::BranchesIgnore => "branches-ignore",
            Self::TagsIgnore => "tags-ignore",
        })
    }
}

impl Filterable for PushEvent {
    type FilterType = PushEventFilter;
}

impl Event for PushEvent {
    fn filter_yaml(&self) -> Option<serde_yaml::Value> {
        if self.filters.is_empty() {
            None
        } else {
            Some(self.filters.to_yaml())
        }
    }

    fn type_yaml(&self) -> Option<serde_yaml::Value> {
        None
    }
}
