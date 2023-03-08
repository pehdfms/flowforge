use flowforge::domain::entities::{
    events::push::{PushEvent, PushEventFilter},
    filter::FilterExpression,
    workflow::Workflow,
};
use yaml_rust::YamlEmitter;

#[test]
fn simple_workflow() {
    let mut first_trigger = PushEvent::default();

    first_trigger.filters.insert(
        PushEventFilter::Branches,
        FilterExpression(String::from("test")),
    );

    let second_trigger = PushEvent::default();

    let workflow = Workflow::default()
        .add_trigger(first_trigger)
        .add_trigger(second_trigger);

    for section in workflow.to_yaml() {
        let mut output = String::new();
        let mut emitter = YamlEmitter::new(&mut output);

        emitter.dump(&section).unwrap();

        println!("{output}");
    }

    assert_eq!(
        workflow.get_triggers().len(),
        2,
        "When adding multiple workflows of the same type, we should only see one"
    )
}
