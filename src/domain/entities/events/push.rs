use crate::domain::{
    common::yaml_conversion::YamlConversion,
    entities::{
        event::{Event, EventTrigger},
        filter::Filterable,
    },
};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PushEvent;

pub enum PushEventFilter {
    Branches,
    Tags,
    BranchesIgnore,
    TagsIgnore,
}

impl Filterable for PushEvent {
    type FilterType = PushEventFilter;
}

impl Event for PushEvent {
    fn get_identifier(&self) -> String {
        String::from("push")
    }
}

impl YamlConversion for PushEvent {
    fn to_yaml(&self) -> String {
        todo!()
    }
}

impl From<PushEvent> for EventTrigger {
    fn from(value: PushEvent) -> Self {
        Self::Push(value)
    }
}
