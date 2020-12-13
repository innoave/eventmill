# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## 0.4.1 : [unreleased]

### changes

* re-export `DomainEventView` in crate root

### docs

* inline documentation of re-exports within crate root

## 0.4.0 : 2020-05-23

### breaking changes

* introduce `DomainEventView` as the result type for reading events from the event store. This
  avoids cloning of data and metadata in some cases. The related traits `Aggregate` and
  `ReceiveEvent` are now getting `DomainEventView` as input instead of `DomainEvent`.
* renamed method `EventSource::read_events` to `EventSource::read`
* added method `read_from_offset` to `EventSource` trait

### features

* the `new` methods of `DomainCommand`, `NewEvent` and `DomainEvent` use `Into` trait to be more
  flexible in passing arguments.
* implement `From` trait for turning a custom command and a reference to the current aggregate
  state into a new `DomainCommand` (see the [turtle example](eventmill-examples/turtle/main.rs)
  on how to use it). 

### fixes

* core applies new events to aggregate before appending them to the event store. this is problematic
  if append to the event store fails.
* implementation of `Clone` and `PartialEq` for `DomainEvent`, `DomainEventView` and `NewEvent` no
  longer require that the type parameter `A` implements these traits.

## 0.3.0 : 2020-05-17

### features

* new `derive(AggregateType)` macro to derive implementations for the `AggregateType`, 
  `WithAggregateId` and `InitializeAggregate` traits.

### examples

* the `counter` example now makes use of the `derive(AggregateType)` macro.

### docs

* add examples to main README.md and eventmill-derive/README.md

## 0.2.0 : 2020-05-16

### breaking changes

* the `payload` field of `NewEvent`, `DomainEvent` and `DomainCommand` has been renamed to `data`
* only the public API is re-exported from the root of the crate. implementations und supplementary
  types have to be imported from the modules where they are defined in.
* the trait `DispatchCommand` requires to define its Output type, which makes it more flexible

### features

* `Generation` implements `From<Sequence>`

### fixes

* `Core` command dispatcher is now usable
  
### internals

* improved tests for the `#[derive(EventType)]` macro
* added more tests for core functions

### examples

* added `counter` example, a minimal "hello world" like one

## 0.1.0 : 2020-05-10

First release
