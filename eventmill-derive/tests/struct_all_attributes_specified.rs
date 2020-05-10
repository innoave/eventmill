use eventmill_derive::EventType;

#[derive(EventType, Debug)]
#[event_type("turtle-turned")]
#[event_type_version("V2")]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {}
