# eventmill-derive &emsp;

[![Latest Release]][crates.io]
[![Documentation]][docs.rs]
[![License]](LICENSE)
[![Rustc Support 1.31+]][Rust 1.31]

[Latest Release]: https://img.shields.io/crates/v/eventmill_derive.svg
[crates.io]: https://crates.io/crates/eventmill_derive
[Documentation]: https://docs.rs/eventmill/badge.svg
[docs.rs]: https://docs.rs/eventmill_derive
[License]: https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg
[MIT]: https://opensource.org/licenses/MIT
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[Rustc Support 1.31+]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

**Derive macros for convenient use of the `eventmill` crate**

You can bring in the macros in two ways. The recommended way by using the `derive` feature of the
main crate:

```toml
[dependencies]
eventmill = { version = "0.2", features = ["derive"] }
```

or the alternative way:

```toml
[dependencies]
eventmill = "0.2"
eventmill_derive = "0.2"
```

The examples below assume you bring in the macros the recommended way using the `derive` feature of
the `eventmill` crate.

## `#[derive(EventType)]`

The `#[derive(EventType)]` macro implements the `EventType` trait for your events. An Event can be
a `struct` or `enum`. We can configure the generated implementation by using the optional attributes
`event_type`, `event_type_version` and `event_source`.

**Here are some examples:**

Implementing an event using a `struct` and specifying all available attributes.

```rust
use eventmill::EventType;

#[derive(EventType, Debug)]
#[event_type_version("V2")]
#[event_source("https://github.com/innoave/eventmill/examples/turtle")]
#[event_type("turtle-turned")]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {
    let turtle = TurtleTurned { angle: 0.42 };

    assert_eq!(turtle.event_type_version(), "V2");
    assert_eq!(
        turtle.event_source(),
        "https://github.com/innoave/eventmill/examples/turtle"
    );
    assert_eq!(turtle.event_type(), "turtle-turned");
}
```

Implementing an event using an `enum` and specifying all available attributes.

```rust
use eventmill::EventType;

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

fn main() {
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
```

If we omit any or all of the attributes the macro uses default values. Note the default names 
returned by the `event_type()` function.

```rust
use eventmill::EventType;

#[derive(EventType, Debug)]
pub enum Turtle {
    Turned(f32),
    Moved { x: i32, y: i32 },
    Stopped,
}

fn main() { 
    let turtle = Turtle::Turned(0.42);
    assert_eq!(turtle.event_type(), "Turtle::Turned");
    
    let turtle = Turtle::Moved { x: 4, y: 2 };
    assert_eq!(turtle.event_type(), "Turtle::Moved");
    
    let turtle = Turtle::Stopped;
    assert_eq!(turtle.event_type(), "Turtle::Stopped");
}
```

We can use any expression that evaluates to `&str` as the values of attributes. E.g. use a `const`
for defining the `event_source` attribute.

```rust
use eventmill::EventType;

const EVENT_NAMESPACE: &str = "https://github.com/innoave/eventmill/examples/turtle";

#[derive(EventType, Debug)]
#[event_source(EVENT_NAMESPACE)]
pub struct TurtleTurned {
    angle: f32,
}

fn main() {
    let turtle = TurtleTurned { angle: 0.42 };

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), EVENT_NAMESPACE);
    assert_eq!(turtle.event_type(), "TurtleTurned");
}
```

## `#[derive(AggregateType)]`

The `#[derive(AggregateType)]` macro implements the `AggregateType` trait for your aggregate types.
Currently, this macro can be used only on `struct` types. We can configure the macro with the two
optional attributes `#[id_field]` and `#[initialize_with_defaults]`.
 
When the `#[id_field]` attribute is specified the macro will additionally implement the 
`WithAggregateId` trait. This attribute has one parameter which is the identifier of the id field
in the struct.    

The `#[initialize_with_defaults]` attribute tells the macro to implement the `InitializeAggregate`
trait using the default values for each field in the struct. This assumes that types used in fields
of the struct implement the `Default` trait. The `#[initialize_with_defaults]` attribute requires 
that the `#[id_field]` attribute is specified for the struct as well.
 
**Here are some examples:**

Derive implementations for the traits `AggregateType`, `WithAggregateId` and `InitializeAggregate`
using the `AggregateType` macro with all optional attributes:

```rust
use eventmill::{AggregateType, InitializeAggregate, WithAggregateId};

#[derive(AggregateType, Debug, PartialEq)]
#[id_field(id)]
#[initialize_with_defaults]
pub struct Turtle {
    id: String,
    x: f32,
    y: f32,
    direction: f32,
    speed: f32,
    pen: bool,
}

#[test]
fn main() {
    let expected_turtle = Turtle {
        id: "4711".to_string(),
        x: Default::default(),
        y: Default::default(),
        direction: Default::default(),
        speed: Default::default(),
        pen: Default::default(),
    };

    let new_turtle = Turtle::initialize("4711".to_string());

    assert_eq!(new_turtle, expected_turtle);
    assert_eq!(new_turtle.aggregate_id(), "4711");
    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
```

Only derive the implementation for `AggregateType`. No additional attributes are to be specified:

```rust
use eventmill::AggregateType;


#[derive(AggregateType, Debug)]
pub struct Turtle {
    id: String,
    x: f32,
    y: f32,
    direction: f32,
    speed: f32,
    pen: bool,
}

#[test]
fn main() {
    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
```

Derive the implementation of the traits `AggregateType` and `WithAggregateId`:

```rust
use eventmill::{AggregateType, WithAggregateId};

#[derive(AggregateType, Debug)]
#[id_field(id)]
pub struct Turtle {
    id: String,
    x: f32,
    y: f32,
    direction: f32,
    speed: f32,
    pen: bool,
}

#[test]
fn main() {
    let turtle = Turtle {
        id: "0815".to_string(),
        x: -0.5,
        y: 0.3,
        direction: 0.42,
        speed: 1.0,
        pen: true,
    };

    assert_eq!(turtle.aggregate_id(), "0815");
    assert_eq!(Turtle::aggregate_type(), "Turtle");
}
```
