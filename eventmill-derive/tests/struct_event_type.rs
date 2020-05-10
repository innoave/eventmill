use eventmill_derive::EventType;

#[test]
fn struct_no_attribute_specified() {
    #[derive(EventType, Debug)]
    pub struct TurtleTurned {
        angle: f32,
    }

    use eventmill::EventType;

    let turtle = TurtleTurned { angle: 0.42 };

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), "");
    assert_eq!(turtle.event_type(), "TurtleTurned");
}

#[test]
fn struct_with_event_type_attribute_omitted() {
    #[derive(EventType, Debug)]
    #[event_type_version("V2")]
    #[event_source("https://github.com/innoave/eventmill/examples/turtle")]
    pub struct TurtleTurned {
        angle: f32,
    }

    use eventmill::EventType;

    let turtle = TurtleTurned { angle: 0.42 };

    assert_eq!(turtle.event_type_version(), "V2");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "TurtleTurned");
}

#[test]
fn struct_all_attributes_specified() {
    #[derive(EventType, Debug)]
    #[event_type_version("V2")]
    #[event_source("https://github.com/innoave/eventmill/examples/turtle")]
    #[event_type("turtle-turned")]
    pub struct TurtleTurned {
        angle: f32,
    }

    use eventmill::EventType;

    let turtle = TurtleTurned { angle: 0.42 };

    assert_eq!(turtle.event_type_version(), "V2");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "turtle-turned");
}
