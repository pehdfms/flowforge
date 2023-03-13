use enum_dispatch::enum_dispatch;
use regex::Regex;
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
pub(crate) fn clean_output(s: &str) -> String {
    let no_nulls = s.replace(" ' '", "");

    let re = Regex::new(r"needs:\s*(-\s*\w+\s*)+\n").unwrap();
    re.replace_all(&no_nulls, |caps: &regex::Captures| {
        let inner_re = Regex::new(r"-\s*").unwrap();
        let inner_input = caps.get(1).unwrap().as_str();
        let inner_output = inner_re.replace_all(inner_input, "").to_string();
        format!("needs: [{}]\n", inner_output)
    })
    .to_string()
}
