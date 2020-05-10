use eventmill_derive::EventType;

#[derive(EventType, Debug)]
pub enum Turtle {
    Turned(f32),
    Moved { x: i32, y: i32 },
    Stopped,
}

fn main() {}
