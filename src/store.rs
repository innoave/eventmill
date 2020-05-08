use crate::query::ReceiveEvent;
use crate::{DomainEvent, EventType, WithAggregateId};

pub trait EventSink<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    type Error;

    fn append(&self, topic: impl Into<String>, event: DomainEvent<E, A>)
        -> Result<(), Self::Error>;

    fn append_batch(
        &self,
        topic: impl Into<String>,
        events: impl IntoIterator<Item = DomainEvent<E, A>>,
    ) -> Result<(), Self::Error>;
}

pub trait EventSource<E, A> {
    type Error;

    fn read_events<R>(&self, topic: &str, subscriber: &mut R) -> Result<(), Self::Error>
    where
        E: EventType,
        A: WithAggregateId,
        R: ReceiveEvent<E, A>;
}
