use std::{collections::HashMap, hash::Hash};

use enum_dispatch::enum_dispatch;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

use crate::domain::entities::filter::FilterExpression;

#[enum_dispatch]
pub trait YamlKey {
    fn get_identifier(&self) -> String;
}

#[enum_dispatch]
pub trait YamlConversion {
    fn to_yaml(&self) -> Yaml;
}

// TODO I'll probably have to completely scrap rust_yaml
// it's a reasonably good library, but it doesn't support the exact
// kind of yaml github actions needs
// for example:
//
// "on":
//   - push
//
// is the kind of code it would generate, but that's not a valid
// github actions yaml file (the hyphen breaks it)
