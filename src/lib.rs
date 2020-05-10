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
