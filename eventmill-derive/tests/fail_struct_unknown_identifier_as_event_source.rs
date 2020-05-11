use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type(UNDEFINED_IDENTIFIER)]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {}
