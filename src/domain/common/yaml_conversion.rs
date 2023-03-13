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

    // This is flimsy code, but I want to kick the bucket of
    // writing my own yaml parser for later
    let re = Regex::new(r"needs:((\s*- \w+\s*?)+)").unwrap();
    re.replace_all(&no_nulls, |caps: &regex::Captures| {
        let inner_input = caps.get(1).unwrap().as_str();

        let inbetween_lines = Regex::new(r"\n\s+- ").unwrap();
        let start = Regex::new(r"\s+- ").unwrap();
        let end = Regex::new(r"\n\s+").unwrap();

        let mut inner_output = start.replace(inner_input, "").to_string();
        inner_output = inbetween_lines.replace_all(&inner_output, ", ").to_string();
        inner_output = end.replace_all(&inner_output, "").to_string();

        format!("needs: [{}]", inner_output)
    })
    .to_string()
}
