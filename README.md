# eventmill &emsp;

[![Latest Release]][crates.io]
[![Documentation]][docs.rs]
[![License]](LICENSE)
[![Rustc Support 1.39+]][Rust 1.39]
[![Build Status]][actions]

[Latest Release]: https://img.shields.io/crates/v/eventmill.svg
[crates.io]: https://crates.io/crates/eventmill
[Documentation]: https://docs.rs/eventmill/badge.svg 
[docs.rs]: https://docs.rs/eventmill
[License]: https://img.shields.io/badge/license-MIT%2FApache_2.0-blue.svg
[MIT]: https://opensource.org/licenses/MIT
[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[Build Status]: https://img.shields.io/github/workflow/status/innoave/eventmill/CI/master
[actions]: https://github.com/innoave/eventmill/actions?query=branch%3Amaster
[Rustc Support 1.39+]: https://img.shields.io/badge/rustc-1.39+-lightgray.svg
[Rust 1.39]: https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html

**Event sourcing and CQRS for Rust applications**

> very much work in progress!

```toml
[dependencies]
eventmill = "0.3"
```

to use the `derive` macros from the `eventmill-derive` crate activate the `derive` feature:

```toml
[dependencies]
eventmill = { version = "0.3", features = ["derive"] }    
```

## Usage example

Define your domain events:

```rust
const EVENT_NAMESPACE: &str = "https://github.com/innoave/eventmill/examples/counter";

#[derive(EventType, Debug, Clone, PartialEq)]
#[event_source(EVENT_NAMESPACE)]
#[event_type_version("V1")]
#[event_type("Incremented")]
struct Incremented;
```

Define your aggregate:

```rust
#[derive(AggregateType, Debug)]
#[id_field(id)]
#[initialize_with_defaults]
struct Counter {
    id: i32,
    hits: u64,
}
```

Implement the business logic for applying events to the aggregate:

```rust
impl Aggregate<Incremented> for Counter {
    fn apply_event(&mut self, _event: &DomainEvent<Incremented, Self>) {
        self.hits += 1;
    }
}
```

Define a command:

```rust
#[derive(Debug, PartialEq)]
struct Increment;
```

Implement the business logic so that the aggregate is able to handle the command:

```rust
impl HandleCommand<Increment, Self> for Counter {
    type Event = Incremented;
    type Error = Infallible;
    type Context = ();

    fn handle_command(
        &self,
        _command: Increment,
        _context: &Self::Context,
    ) -> Result<Vec<NewEvent<Self::Event, Counter>>, Self::Error> {
        Ok(vec![NewEvent {
            aggregate_id: self.id,
            data: Incremented,
        }])
    }
}
```

Bringing it all together using the `Core` dispatcher:

```rust
fn main() {
    let event_store = InMemoryStore::new();
    let core = Core::new(event_store);

    let aggregate_id = 4711;

    let increment_command = DomainCommand {
        aggregate_id,
        aggregate_generation: Generation::default(),
        data: Increment,
    };

    let versioned_counter: VersionedAggregate<Counter> = core
        .dispatch_command(increment_command, &())
        .expect("counter incremented");

    assert_eq!(versioned_counter.state().hits, 1);
}
```

These code samples are taken from the `counter` example. Take a look at [`eventmill-examples`] for
more insights.

## TODO

* [X] define basic abstractions and API
* [X] provide a first example on how it looks like to use the API
* [ ] make more examples to polish the API
* [ ] write rust-doc for the API
* [ ] support async/await or switch to async as the only option
* [ ] consider providing default implementations for eventstores and other building blocks
* [ ] ...

[`eventmill-examples`]: https://github.com/innoave/eventmill/tree/master/eventmill-examples
