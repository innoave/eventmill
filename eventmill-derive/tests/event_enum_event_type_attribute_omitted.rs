use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type_version("V2")]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
pub enum Turtle {
    Turned(f32),
    Moved { x: i32, y: i32 },
    Stopped,
}

#[test]
fn main() {
    use eventmill::EventType;

    let turtle = Turtle::Stopped;

    assert_eq!(turtle.event_type_version(), "V2");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "Turtle::Stopped");

    let turtle = Turtle::Turned(0.42);
    assert_eq!(turtle.event_type(), "Turtle::Turned");

    let turtle = Turtle::Moved { x: 4, y: 2 };
    assert_eq!(turtle.event_type(), "Turtle::Moved");
}
