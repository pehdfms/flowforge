use super::event::Event;

pub struct FilterExpression {}

pub trait Filterable: Event {
    type FilterType;
}
