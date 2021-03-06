use eventmill_derive::EventType;

const PROTOCOL_VERSION: &str = "V5";

#[derive(EventType, Debug)]
#[event_type_version(PROTOCOL_VERSION)]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
pub enum Turtle {
    #[event_type("turtle-turned")]
    Turned(f32),
    #[event_type("turtle-moved")]
    Moved { x: i32, y: i32 },
    #[event_type("turtle-stopped")]
    Stopped,
}

#[test]
fn main() {
    use eventmill::EventType;

    let turtle = Turtle::Stopped;

    assert_eq!(turtle.event_type_version(), "V5");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "turtle-stopped");

    let turtle = Turtle::Turned(0.42);
    assert_eq!(turtle.event_type(), "turtle-turned");

    let turtle = Turtle::Moved { x: 4, y: 2 };
    assert_eq!(turtle.event_type(), "turtle-moved");
}
