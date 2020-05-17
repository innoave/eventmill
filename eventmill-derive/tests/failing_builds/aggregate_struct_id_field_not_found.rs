use eventmill_derive::AggregateType;

#[derive(AggregateType, Debug)]
#[id_field(turtle_id)]
pub struct Turtle {
    id: String,
    x: f32,
    y: f32,
    direction: f32,
    speed: f32,
    pen: bool,
}

fn main() {}
