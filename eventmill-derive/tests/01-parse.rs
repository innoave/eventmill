const EVENT_NAMESPACE: &str = "https://github.com/innoave/eventmill/examples/turtle";

use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type("TurtleTurned")]
#[event_type_version("V1")]
#[event_source(EVENT_NAMESPACE)]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {}
