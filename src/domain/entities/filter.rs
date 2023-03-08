use crate::domain::common::yaml_conversion::YamlKey;

use super::event::Event;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct FilterExpression {}

pub trait Filterable: Event {
    type FilterType: YamlKey;
}
