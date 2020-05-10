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
#![doc(html_root_url = "https://docs.rs/eventmill/0.1.0")]
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

mod aggregate;
mod command;
mod dispatch;
mod event;
pub mod inmemory_store;
mod metadata;
mod query;
mod store;
pub mod test_support;

pub use aggregate::*;
pub use command::*;
pub use dispatch::*;
pub use event::*;
pub use metadata::*;
pub use query::*;
pub use store::*;

#[cfg(feature = "derive")]
pub use eventmill_derive::*;
