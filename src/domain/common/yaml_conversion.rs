use enum_dispatch::enum_dispatch;
use serde_yaml::Value;

#[enum_dispatch]
pub trait YamlKey {
    fn get_identifier(&self) -> String;
}

#[enum_dispatch]
pub trait YamlConversion {
    fn to_yaml(&self) -> Value;
}

pub fn empty_keys(keys: Vec<Value>) -> Value {
    Value::String(
        keys.into_iter()
            .map(|key| match key {
                Value::String(s) => s,
                _ => panic!(
                    "impossible to generate empty keys from any Value variant other than String"
                ),
            })
            .collect::<Vec<String>>()
            .join("\n"),
    )
}
