use std::hash::{Hash, Hasher};

use enum_dispatch::enum_dispatch;

use crate::domain::common::yaml_conversion::{YamlConversion, YamlKey};

use super::events::push::PushEvent;

pub trait TypedEvent: Event {
    type ActivityType;
}

#[enum_dispatch]
pub trait Event: YamlConversion {
    fn filter_yaml(&self) -> Option<serde_yaml::Value>;
    fn type_yaml(&self) -> Option<serde_yaml::Value>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[enum_dispatch(Event, YamlKey)]
pub enum EventTrigger {
    PushEvent,
}

// TODO switch to derive hash by variant crate
impl Hash for EventTrigger {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let type_id = std::any::TypeId::of::<Self>();
        let discriminant = std::mem::discriminant(self);

        discriminant.hash(state);
        type_id.hash(state);
    }
}

impl<T: Event + YamlKey> YamlConversion for T {
    fn to_yaml(&self) -> serde_yaml::Value {
        todo!()
    }
}
