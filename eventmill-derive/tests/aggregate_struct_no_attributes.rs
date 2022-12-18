use eventmill_derive::AggregateType;

#[derive(AggregateType, Debug)]
pub struct Turtle {}

#[test]
fn main() {
    use eventmill::AggregateType;

    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
