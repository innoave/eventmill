use eventmill_derive::EventType;

#[test]
fn enum_no_attribute_specified() {
    #[derive(EventType, Debug)]
    pub enum Turtle {
        Turned(f32),
        Moved { x: i32, y: i32 },
        Stopped,
    }

    use eventmill::EventType;

    let turtle = Turtle::Stopped;

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), "");
    assert_eq!(turtle.event_type(), "Turtle::Stopped");

    let turtle = Turtle::Turned(0.42);
    assert_eq!(turtle.event_type(), "Turtle::Turned");

    let turtle = Turtle::Moved { x: 4, y: 2 };
    assert_eq!(turtle.event_type(), "Turtle::Moved");
}

#[test]
fn enum_with_event_type_attribute_omitted() {
    #[derive(EventType, Debug)]
    #[event_type_version("V2")]
    #[event_source("https://github.com/innoave/eventmill/examples/turtle")]
    pub enum Turtle {
        Turned(f32),
        Moved { x: i32, y: i32 },
        Stopped,
    }

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

#[test]
fn enum_with_event_type_attribute_on_variants() {
    #[derive(EventType, Debug)]
    #[event_type_version("V2")]
    #[event_source("https://github.com/innoave/eventmill/examples/turtle")]
    pub enum Turtle {
        #[event_type("turtle-turned")]
        Turned(f32),
        #[event_type("turtle-moved")]
        Moved { x: i32, y: i32 },
        #[event_type("turtle-stopped")]
        Stopped,
    }

    use eventmill::EventType;

    let turtle = Turtle::Stopped;

    assert_eq!(turtle.event_type_version(), "V2");
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
