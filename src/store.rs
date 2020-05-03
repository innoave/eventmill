use crate::{DomainEvent, EventType, WithAggregateId};

pub trait EventStore<A>
where
    A: WithAggregateId,
{
    type Error;

    fn append<E>(&mut self, event: DomainEvent<E, A>, stream: &str) -> Result<(), Self::Error>
    where
        E: EventType;
}
