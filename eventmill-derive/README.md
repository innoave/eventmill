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

```toml
[dependencies]
eventmill_derive = "0.1"    
```

## `#[derive(EventType)]`

The `#[derive(EventType)]` macro implements the `EventType` trait for your events. An Event can be
a `struct` or `enum`. We can configure the generated implementation by using the optional attributes
`event_type`, `event_type_version` and `event_source`.

Implementing an event using a `struct` and specifying all available attributes.

```rust
#[derive(EventType, Debug)]
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
    assert_eq!(turtle.event_type(), "TurtleTurned");
}
```

Implementing an event using an `enum` and specifying all available attributes.

```rust
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
```

If we omit any or all of the attributes the macro uses default values. Note the default names 
returned by the `event_type()` function.

```rust
#[derive(eventmill_derive::EventType, Debug)]
pub enum Turtle {
    Turned(f32),
    Moved { x: i32, y: i32 },
    Stopped,
}

fn main() { 
    use eventmill::EventType;

    assert_eq!(turtle.event_type_version(), "V0");
    assert_eq!(turtle.event_source(), "");
    
    let turtle = Turtle::Turned(0.42);
    assert_eq!(turtle.event_type(), "Turtle::Turned");
    
    let turtle = Turtle::Moved { x: 4, y: 2 };
    assert_eq!(turtle.event_type(), "Turtle::Moved");
    
    let turtle = Turtle::Stopped;
    assert_eq!(turtle.event_type(), "Turtle::Stopped");
}
```
