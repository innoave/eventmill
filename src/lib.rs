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

pub trait EventType {
    fn event_type_version(&self) -> &str;
    fn event_type(&self) -> &str;
    fn event_source(&self) -> &str;
}

pub trait AggregateState {
    fn generation(&self) -> u64;
}

pub trait Aggregate<C>
where
    Self: Sized,
{
    type Event: 'static + EventType;

    fn handle_command(&self, command: C) -> Self::Event;

    fn apply_event(self, event: &Self::Event) -> Self;

    fn apply_all_events<'a>(self, events: impl IntoIterator<Item = &'a Self::Event>) -> Self {
        events
            .into_iter()
            .fold(self, |acc_state, event| acc_state.apply_event(event))
    }
}

pub trait EventStore {
    type Error;

    fn append<E>(&mut self, event: E, stream: &str) -> Result<(), Self::Error>
    where
        E: EventType;
}

pub trait Dispatch {
    fn dispatch<E>(&self, event: E, stream: &str)
    where
        E: EventType;
}
