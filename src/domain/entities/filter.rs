use std::collections::HashMap;

use crate::domain::common::yaml_conversion::{YamlConversion, YamlKey};

use super::event::Event;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct FilterExpression(pub String);

pub trait Filterable: Event {
    type FilterType: YamlKey;
}

impl YamlConversion for FilterExpression {
    fn to_yaml(&self) -> serde_yaml::Value {
        todo!()
    }
}

impl<T: YamlKey> YamlConversion for HashMap<T, FilterExpression> {
    fn to_yaml(&self) -> serde_yaml::Value {
        todo!()
    }
}
