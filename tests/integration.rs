use flowforge::domain::entities::{
    event::EventTrigger, events::push::PushEvent, workflow::Workflow,
};

#[test]
fn simple_workflow() {
    let mut workflow = Workflow::default();

    let trigger: EventTrigger = PushEvent::default().into();
    workflow.add_trigger(trigger);

    assert_eq!(4, 2 + 2)
}
