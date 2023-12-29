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
#![deny(unsafe_code, unstable_features)]
#![warn(
    bare_trait_objects,
    deprecated,
    explicit_outlives_requirements,
    missing_copy_implementations,
    missing_debug_implementations,
    noop_method_call,
    rust_2018_idioms,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]
#![warn(unused_crate_dependencies, unused_extern_crates)]
//#![deny(missing_docs)]  //TODO uncomment when working on docs
//#![warn(variant_size_differences)] // enable when working on performance

pub mod aggregate;
pub mod command;
pub mod dispatch;
pub mod event;
pub mod inmemory_store;
pub mod metadata;
pub mod query;
pub mod store;
pub mod test_support;
pub mod time;

//
// Re-export of types and functions of the public API
//
// We should not re-export:
//
// * specific implementations of the API, e.g. `InMemoryStore`.
// * types or functions with names that would be ambiguous if exported without
//   the module name.
//
#[doc(inline)]
pub use crate::aggregate::{
    Aggregate, AggregateIdOf, AggregateState, AggregateType, Generation, InitializeAggregate,
    VersionedAggregate, WithAggregateId,
};
#[doc(inline)]
pub use crate::command::{DomainCommand, HandleCommand};
#[doc(inline)]
pub use crate::dispatch::{DispatchCommand, DispatchEvent};
#[doc(inline)]
pub use crate::event::{DomainEvent, DomainEventView, EventType, NewEvent, Sequence};
#[doc(inline)]
pub use crate::metadata::Metadata;
#[doc(inline)]
pub use crate::query::ReceiveEvent;
#[doc(inline)]
pub use crate::store::{EventSink, EventSinkError, EventSource, EventSourceError};

// Export derive macros
#[cfg(feature = "derive")]
pub use eventmill_derive::{AggregateType, EventType};

#[cfg(test)]
mod tests {
    use version_sync as _;
}
