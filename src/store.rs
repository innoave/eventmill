use crate::{EventType, NewEvent};

pub trait EventStore {
    type Error;

    fn append<E>(&mut self, event: NewEvent<E>, stream: &str) -> Result<(), Self::Error>
    where
        E: EventType;
}
