use enum_dispatch::enum_dispatch;
use regex::Regex;
use serde_yaml::Value;

#[enum_dispatch]
pub trait YamlKey {
    fn get_identifier(&self) -> String;
}

#[enum_dispatch]
pub trait YamlConversion {
    fn to_yaml(&self) -> Value;
}

// TODO This sucks! Terrible! Awful! But the solution is to write
// a brand new yaml parsing library specific to github actions
// (which I'll eventually have to do anyway)
// For now this is fine for prototyping
pub fn empty_yaml() -> Value {
    Value::String(" ".to_string())
}

pub fn remove_empty_yaml(s: &str) -> String {
    let re = Regex::new(r#" ' '"#).unwrap();
    re.replace_all(s, "").to_string()
}
