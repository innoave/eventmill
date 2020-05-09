use crate::query::ReceiveEvent;
use crate::{AggregateIdOf, DomainEvent, EventType, WithAggregateId};

pub trait EventSink<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    type Error;

    fn append(&self, event: DomainEvent<E, A>) -> Result<(), Self::Error>;

    fn append_batch(
        &self,
        events: impl IntoIterator<Item = DomainEvent<E, A>>,
    ) -> Result<(), Self::Error>;
}

pub type EventSinkError<S, E, A> = <S as EventSink<E, A>>::Error;

pub trait EventSource<E, A> {
    type Error;

    fn read_events<R>(
        &self,
        aggregate_id: &AggregateIdOf<A>,
        subscriber: &mut R,
    ) -> Result<(), Self::Error>
    where
        E: EventType,
        A: WithAggregateId,
        R: ReceiveEvent<E, A>;
}

pub type EventSourceError<S, E, A> = <S as EventSource<E, A>>::Error;
