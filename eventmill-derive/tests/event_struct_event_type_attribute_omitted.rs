use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type_version("V3")]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
pub struct TurtleTurned {}

#[test]
fn main() {
    use eventmill::EventType;

    let turtle = TurtleTurned {};

    assert_eq!(turtle.event_type_version(), "V3");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "TurtleTurned");
}
