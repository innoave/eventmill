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
    use eventmill::WithAggregateId;

    let turtle = Turtle {
        id: "0815".to_string(),
        x: 0.0,
        y: 0.0,
        direction: 0.0,
        speed: 0.0,
        pen: false,
    };

    assert_eq!(turtle.aggregate_id(), "0815");
}
