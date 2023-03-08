use std::hash::{Hash, Hasher};

use enum_dispatch::enum_dispatch;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::domain::common::yaml_conversion::{YamlConversion, YamlKey};

use super::{events::push::PushEvent, filter::Filterable};

pub trait TypedEvent: Event {
    type ActivityType;
}

#[enum_dispatch]
pub trait Event: YamlConversion {
    fn filter_yaml(&self) -> Option<Yaml>;
    fn type_yaml(&self) -> Option<Yaml>;
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
    fn to_yaml(&self) -> Yaml {
        let identifier = Yaml::String(self.get_identifier());
        let mut options = Vec::new();

        if let Some(yaml) = self.filter_yaml() {
            options.push(yaml);
        }

        if let Some(yaml) = self.type_yaml() {
            options.push(yaml);
        }

        if options.is_empty() {
            identifier
        } else {
            let mut map = LinkedHashMap::new();
            map.insert(identifier, Yaml::Array(options));

            Yaml::Hash(map)
        }
    }
}
