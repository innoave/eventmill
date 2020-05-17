use eventmill_derive::AggregateType;

#[derive(AggregateType, Debug, PartialEq)]
#[id_field(id)]
#[initialize_with_defaults]
pub struct Turtle {
    id: String,
    x: f32,
    y: f32,
    direction: f32,
    speed: f32,
    pen: bool,
}

#[test]
fn main() {
    use eventmill::{AggregateType, InitializeAggregate, WithAggregateId};

    let default_turtle = Turtle {
        id: "4711".to_string(),
        x: Default::default(),
        y: Default::default(),
        direction: Default::default(),
        speed: Default::default(),
        pen: Default::default(),
    };

    let new_turtle = Turtle::initialize("4711".to_string());

    assert_eq!(new_turtle, default_turtle);
    assert_eq!(new_turtle.aggregate_id(), "4711");
    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
