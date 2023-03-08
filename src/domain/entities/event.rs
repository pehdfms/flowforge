use std::hash::{Hash, Hasher};

use crate::domain::common::yaml_conversion::YamlConversion;

use super::events::push::PushEvent;

pub trait TypedEvent: Event {
    type ActivityType;
}

pub trait Event: YamlConversion {
    fn get_identifier(&self) -> String;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum EventTrigger {
    Push(PushEvent),
}

// Lots of boilerplate for a little more convenience to the end user
// You can't say I'm not nice :)
impl Event for EventTrigger {
    fn get_identifier(&self) -> String {
        match self {
            EventTrigger::Push(e) => e.get_identifier(),
        }
    }
}

impl YamlConversion for EventTrigger {
    fn to_yaml(&self) -> String {
        match self {
            EventTrigger::Push(e) => e.to_yaml(),
        }
    }
}

impl Hash for EventTrigger {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let type_id = std::any::TypeId::of::<Self>();
        let discriminant = std::mem::discriminant(self);

        discriminant.hash(state);
        type_id.hash(state);
    }
}
