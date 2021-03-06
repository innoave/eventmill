use eventmill_derive::AggregateType;

#[derive(AggregateType, Debug)]
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
    use eventmill::AggregateType;

    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
