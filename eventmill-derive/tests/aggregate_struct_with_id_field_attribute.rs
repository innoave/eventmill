use eventmill_derive::AggregateType;

#[derive(AggregateType, Debug)]
#[id_field(id)]
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
    use eventmill::{AggregateType, WithAggregateId};

    let turtle = Turtle {
        id: "0815".to_string(),
        x: -0.5,
        y: 0.3,
        direction: 0.42,
        speed: 1.0,
        pen: true,
    };

    assert_eq!(turtle.aggregate_id(), "0815");
    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
