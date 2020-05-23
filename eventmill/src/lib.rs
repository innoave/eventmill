//!
//! ## Abstractions
//!
//! Event applied to an aggregate state results in a modified aggregate state:
//!
//! ```text
//! (aggregate, Event) -> aggregate'
//! ```
//!
//! Commands are handled by aggregates resulting in a zero, one or several events:
//!
//! ```text
//! (Aggregate, Command) -> Vec<NewEvent>
//! ```
//!
//! ```text
//! (NewEvent, Store) -> DomainEvent
//! ```
//!
//! Aggregates have a generation property. The generation of an aggregate enumerates the number of
//! modifications through a sequence of applied events.
//!
//! Applying an event to an aggregate advances its state from the current generation n to the next
//! generation n+1:
//!
//! ```text
//! (Aggregate(n), DomainEvent) -> Aggregate(n+1)
//! ```
//!
//! Aggregates can be replayed by applying all stored events to a newly initialized aggregate:
//!
//! ```text
//! (initialized aggregate, &[DomainEvent]) -> current aggregate
//! ```
//!
//!
//! ## Dispatching a Command
//!
//! Given a command we need to find the right aggregate instance to handle it.
//!
//! When dispatcher gets the aggregate instance it calls the handle_command function. The returned
//! list of events (`Vec<NewEvent>`) will be enriched with the address properties of the aggregate to
//! get the related `DomainEvent`s.
//!
#![doc(html_root_url = "https://docs.rs/eventmill/0.4.0")]
//#![deny(missing_docs)]
#![deny(unsafe_code, unstable_features)]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

pub mod aggregate;
pub mod command;
pub mod dispatch;
pub mod event;
pub mod inmemory_store;
pub mod metadata;
pub mod query;
pub mod store;
pub mod test_support;

//
// Re-export of types and functions of the public API
//
// We should not re-export:
//
// * specific implementations of the API, e.g. `InMemoryStore`.
// * types or functions with names that would be ambiguous if exported without
//   the module name.
//
pub use crate::aggregate::{
    Aggregate, AggregateIdOf, AggregateState, AggregateType, Generation, InitializeAggregate,
    VersionedAggregate, WithAggregateId,
};
pub use crate::command::{DomainCommand, HandleCommand};
pub use crate::dispatch::{DispatchCommand, DispatchEvent};
pub use crate::event::{DomainEvent, EventType, NewEvent, Sequence};
pub use crate::metadata::Metadata;
pub use crate::query::ReceiveEvent;
pub use crate::store::{EventSink, EventSinkError, EventSource, EventSourceError};

// Export derive macros
#[cfg(feature = "derive")]
pub use eventmill_derive::{AggregateType, EventType};
