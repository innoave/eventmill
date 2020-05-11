use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type("turtle-turned")]
#[event_type_version("V2")]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {
    use eventmill::EventType;

    let turtle = TurtleTurned { angle: 0.42 };

    assert_eq!(turtle.event_type_version(), "V2");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "turtle-turned");
}
