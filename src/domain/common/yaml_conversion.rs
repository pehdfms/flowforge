use enum_dispatch::enum_dispatch;
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

#[enum_dispatch]
pub trait YamlKey {
    fn get_identifier(&self) -> String;
}

#[enum_dispatch]
pub trait YamlConversion {
    fn to_yaml(&self) -> String;
}
