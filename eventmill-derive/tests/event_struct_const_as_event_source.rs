use eventmill_derive::EventType;

const EVENT_NAMESPACE: &str = "https://github.com/innoave/eventmill/examples/turtle";

#[derive(EventType, Debug)]
#[event_source(EVENT_NAMESPACE)]
pub struct TurtleTurned {}

#[test]
fn main() {
    use eventmill::EventType;

    let turtle = TurtleTurned {};

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), EVENT_NAMESPACE);
    assert_eq!(turtle.event_type(), "TurtleTurned");
}
