use enum_dispatch::enum_dispatch;
use serde_yaml::Value;

#[enum_dispatch]
pub(crate) trait YamlKey {
    fn get_identifier(&self) -> String;
}

#[enum_dispatch]
pub(crate) trait YamlConversion {
    fn to_yaml(&self) -> Value;
}

// TODO This sucks! Terrible! Awful! But the solution is to write
// a brand new yaml parsing library specific to github actions
// (which I'll eventually have to do anyway)
// For now this is fine for prototyping
#[must_use]
pub(crate) fn empty_yaml() -> Value {
    Value::String(" ".to_string())
}

#[must_use]
pub(crate) fn remove_empty_yaml(s: &str) -> String {
    s.replace(" ' '", "")
}
