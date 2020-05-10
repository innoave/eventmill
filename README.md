# eventmill &emsp;

[![Latest Release]][crates.io]
[![Documentation]][docs.rs]
[![License]](LICENSE)
[![Rustc Support 1.31+]][Rust 1.31]
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
[Rustc Support 1.31+]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[Rust 1.31]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

**Event sourcing and CQRS for Rust applications**

```toml
[dependencies]
eventmill = "0.1"    
```


```text
    (state, event) -> state
```

```text
    (Aggregate, Command) -> Vec<NewEvent>
```
```text
    (NewEvent, Store) -> DomainEvent
```
```text
    (Aggregate(n), DomainEvent) -> Aggregate(n+1)
```


## Dispatch a Command

Given a command we need to find the right aggregate instance to handle it.

When dispatcher gets the aggregate instance it calls the handle_command function. The returned
list of events (`Vec<NewEvent>`) will be enriched with the address properties of the aggregate to
get the related `DomainEvent`.

```rust
    trait DispatchCommand<C, S> {
        type Aggregate: HandleCommand<C, Self> + WithAggregateId;
        type Error;

        fn dispatch_command(command: C, context: S) -> Result<Vec<DomainEvent<E, AggregateIdOf<Self::Aggregate>>>, Self::Error>;
    }
```  
