use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type_version("V1")]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {}
