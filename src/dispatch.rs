use crate::{DomainEvent, EventType, WithAggregateId};

pub trait Dispatch {
    fn dispatch<E, A>(&self, event: DomainEvent<E, A>, stream: &str)
    where
        E: EventType,
        A: WithAggregateId;
}
