use crate::{EventType, StoredEvent};

pub trait Dispatch {
    fn dispatch<E>(&self, event: StoredEvent<E>, stream: &str)
    where
        E: EventType;
}
