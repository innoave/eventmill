use eventmill_derive::EventType;

#[derive(EventType, Debug)]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {}
