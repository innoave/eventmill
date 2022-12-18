use eventmill_derive::EventType;

#[derive(EventType, Debug)]
pub struct TurtleTurned {}

#[test]
fn main() {
    use eventmill::EventType;

    let turtle = TurtleTurned {};

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), "");
    assert_eq!(turtle.event_type(), "TurtleTurned");
}
