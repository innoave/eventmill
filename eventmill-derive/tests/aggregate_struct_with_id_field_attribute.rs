use eventmill_derive::AggregateType;

#[derive(AggregateType, Debug)]
#[id_field(id)]
pub struct Turtle {
    id: String,
}

#[test]
fn main() {
    use eventmill::{AggregateType, WithAggregateId};

    let turtle = Turtle {
        id: "0815".to_string(),
    };

    assert_eq!(turtle.aggregate_id(), "0815");
    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
