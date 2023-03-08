use std::collections::HashMap;

use flowforge::domain::entities::{events::push::PushEvent, workflow::Workflow};
use linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlEmitter};

#[test]
fn simple_workflow() {
    let first_trigger = PushEvent::default();
    let second_trigger = PushEvent::default();

    let workflow = Workflow::default()
        .add_trigger(first_trigger)
        .add_trigger(second_trigger);

    assert_eq!(
        workflow.get_triggers().len(),
        1,
        "When adding multiple workflows of the same type, we should only see one"
    )
}

#[test]
fn yaml() {
    let a = Yaml::Array(vec![Yaml::Integer(1), Yaml::Integer(2)]);
    let map = LinkedHashMap::from_iter(vec![(Yaml::String(String::from("filters")), a)]);

    let b = Yaml::Hash(map);

    let mut output = String::new();
    let mut emitter = YamlEmitter::new(&mut output);

    emitter.dump(&b).unwrap();

    println!("{}", output);
    assert!(false);
}
