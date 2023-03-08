use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::domain::common::yaml_conversion::{YamlConversion, YamlKey};

use super::event::Event;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct FilterExpression(pub String);

pub trait Filterable: Event {
    type FilterType: YamlKey;
}

impl YamlConversion for FilterExpression {
    fn to_yaml(&self) -> yaml_rust::Yaml {
        Yaml::String(self.0.clone())
    }
}

impl<T: YamlKey> YamlConversion for HashMap<T, FilterExpression> {
    fn to_yaml(&self) -> Yaml {
        let map = LinkedHashMap::from_iter(
            self.iter()
                .map(|(k, v)| (Yaml::String(k.get_identifier()), v.to_yaml())),
        );

        Yaml::Hash(map)
    }
}
