use crate::{
    AggregateIdOf, AggregateType, DomainEvent, EventSink, EventSource, EventType, ReceiveEvent,
    WithAggregateId,
};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::{Arc, RwLock};

type EventMap<E, A> = HashMap<String, Vec<DomainEvent<E, A>>>;

pub type InMemoryStoreError = Error;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("can not acquire read access, cause: {0}")]
    NoReadAccess(String),
    #[error("can not acquire write access, cause: {0}")]
    NoWriteAccess(String),
}

#[derive(Debug, Default)]
pub struct InMemoryStore<E, A>
where
    A: WithAggregateId,
{
    events: Arc<RwLock<EventMap<E, A>>>,
}

impl<E, A> InMemoryStore<E, A>
where
    A: AggregateType + WithAggregateId,
    AggregateIdOf<A>: Display,
{
    pub fn with_events(events: impl IntoIterator<Item = DomainEvent<E, A>>) -> Self {
        let mut event_map = EventMap::with_capacity(4);
        events.into_iter().for_each(|ev| {
            event_map
                .entry(ev.aggregate_id.to_string())
                .or_insert_with(|| Vec::with_capacity(4))
                .push(ev)
        });
        Self {
            events: Arc::new(RwLock::new(event_map)),
        }
    }
}

impl<E, A> EventSink<E, A> for InMemoryStore<E, A>
where
    E: EventType,
    A: AggregateType + WithAggregateId,
    AggregateIdOf<A>: Display,
{
    type Error = Error;

    fn append(&self, event: DomainEvent<E, A>) -> Result<(), Self::Error> {
        let mut event_map = self
            .events
            .write()
            .map_err(|err| Error::NoWriteAccess(err.to_string()))?;
        event_map
            .entry(event.aggregate_id.to_string())
            .or_insert_with(|| Vec::with_capacity(4))
            .push(event);
        Ok(())
    }

    fn append_batch(
        &self,
        events: impl IntoIterator<Item = DomainEvent<E, A>>,
    ) -> Result<(), Self::Error> {
        let mut event_map = self
            .events
            .write()
            .map_err(|err| Error::NoWriteAccess(err.to_string()))?;
        events.into_iter().for_each(|ev| {
            event_map
                .entry(ev.aggregate_id.to_string())
                .or_insert_with(|| Vec::with_capacity(4))
                .push(ev)
        });
        Ok(())
    }
}

impl<E, A> EventSource<E, A> for InMemoryStore<E, A>
where
    A: WithAggregateId,
    AggregateIdOf<A>: Display,
{
    type Error = Error;

    fn read_events<R>(
        &self,
        aggregate_id: &AggregateIdOf<A>,
        subscriber: &mut R,
    ) -> Result<(), Self::Error>
    where
        E: EventType,
        A: WithAggregateId,
        R: ReceiveEvent<E, A>,
    {
        let event_map = self
            .events
            .read()
            .map_err(|err| Error::NoReadAccess(err.to_string()))?;
        event_map
            .get(&aggregate_id.to_string())
            .iter()
            .for_each(|events| events.iter().for_each(|ev| subscriber.receive_event(ev)));
        Ok(())
    }
}
