use crate::aggregate::{AggregateIdOf, WithAggregateId};
use crate::event::{DomainEvent, EventType, Sequence};
use crate::query::ReceiveEvent;

pub trait EventSink<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    type Error: std::error::Error;

    fn append(&self, event: DomainEvent<E, A>) -> Result<(), Self::Error>;

    fn append_batch(
        &self,
        events: impl IntoIterator<Item = DomainEvent<E, A>>,
    ) -> Result<(), Self::Error>;
}

pub type EventSinkError<S, E, A> = <S as EventSink<E, A>>::Error;

pub trait EventSource<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    type Error: std::error::Error;

    fn read<R>(
        &self,
        aggregate_id: &AggregateIdOf<A>,
        subscriber: &mut R,
    ) -> Result<(), Self::Error>
    where
        R: ReceiveEvent<E, A>;

    fn read_from_offset<R>(
        &self,
        aggregate_id: &AggregateIdOf<A>,
        offset: Sequence,
        subscriber: &mut R,
    ) -> Result<(), Self::Error>
    where
        R: ReceiveEvent<E, A>;
}

pub type EventSourceError<S, E, A> = <S as EventSource<E, A>>::Error;
