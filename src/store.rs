use crate::query::ReceiveEvent;
use crate::{DomainEvent, EventType, WithAggregateId};

pub trait EventSink<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    type Error;

    fn append(&self, event: DomainEvent<E, A>, stream: &str) -> Result<(), Self::Error>;

    fn append_all(&self, events: Vec<DomainEvent<E, A>>) -> Result<(), Self::Error>;
}

pub trait EventSource {
    type Error;

    fn read_events<E, A, R>(&self, stream: &str, subscriber: R) -> Result<(), Self::Error>
    where
        E: EventType,
        A: WithAggregateId,
        R: ReceiveEvent<E, A>;
}
