#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

mod aggregate;
mod dispatch;
mod event;
mod store;

pub use aggregate::*;
pub use dispatch::*;
pub use event::*;
pub use store::*;
