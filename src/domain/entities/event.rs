use std::hash::{Hash, Hasher};

use enum_dispatch::enum_dispatch;
use serde_yaml::{Mapping, Value};

use crate::domain::common::yaml_conversion::{empty_yaml, YamlConversion, YamlKey};

use super::events::push::PushEvent;

pub(crate) trait TypedEvent: Event {
    type ActivityType;
}

#[enum_dispatch]
pub(crate) trait Event: YamlConversion {
    fn filter_yaml(&self) -> Option<Value>;
    fn type_yaml(&self) -> Option<Value>;
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
    fn to_yaml(&self) -> Value {
        let mut map = Mapping::new();

        if let Some(yaml) = self.filter_yaml() {
            map.extend(yaml.as_mapping().unwrap().clone());
        }

        if let Some(yaml) = self.type_yaml() {
            map.extend(yaml.as_mapping().unwrap().clone());
        }

        if map.is_empty() {
            map.insert(Value::from(self.get_identifier()), empty_yaml());
            Value::from(map)
        } else {
            Value::from(Mapping::from_iter(vec![(
                Value::from(self.get_identifier()),
                Value::from(map),
            )]))
        }
    }
}
