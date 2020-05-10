use eventmill_derive::EventType;

#[derive(EventType, Debug)]
pub enum Turtle {
    Turned(f32),
    Moved { x: i32, y: i32 },
    Stopped,
}

fn main() {
    use eventmill::EventType;

    let turtle = Turtle::Stopped;

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), "");
    assert_eq!(turtle.event_type(), "Turtle::Stopped 2");
}
