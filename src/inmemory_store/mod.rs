use crate::{
    AggregateType, DomainEvent, EventSink, EventSource, EventType, ReceiveEvent, WithAggregateId,
};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::{Arc, RwLock};

type EventMap<E, A> = HashMap<String, Vec<DomainEvent<E, A>>>;

#[derive(Debug)]
pub struct InMemoryStore<E, A>
where
    A: WithAggregateId,
{
    events: Arc<RwLock<EventMap<E, A>>>,
}

impl<E, A> InMemoryStore<E, A>
where
    A: AggregateType + WithAggregateId,
{
    pub fn with_events(
        events: impl IntoIterator<
            Item = (
                impl Into<String>,
                impl IntoIterator<Item = DomainEvent<E, A>>,
            ),
        >,
    ) -> Self {
        let event_map = EventMap::from_iter(
            events
                .into_iter()
                .map(|(topic, events)| (topic.into(), Vec::from_iter(events.into_iter()))),
        );

        Self {
            events: Arc::new(RwLock::new(event_map)),
        }
    }
}

impl<E, A> EventSink<E, A> for InMemoryStore<E, A>
where
    E: EventType,
    A: AggregateType + WithAggregateId,
{
    type Error = String;

    fn append(
        &self,
        topic: impl Into<String>,
        event: DomainEvent<E, A>,
    ) -> Result<(), Self::Error> {
        let mut event_map = self.events.write().map_err(|err| err.to_string())?;
        event_map
            .entry(topic.into())
            .or_insert_with(|| Vec::with_capacity(1))
            .push(event);
        Ok(())
    }

    fn append_batch(
        &self,
        topic: impl Into<String>,
        events: impl IntoIterator<Item = DomainEvent<E, A>>,
    ) -> Result<(), Self::Error> {
        let mut event_map = self.events.write().map_err(|err| err.to_string())?;
        let event_iter = events.into_iter();
        let (min, maybe_max) = event_iter.size_hint();
        let estimated_capacity = if let Some(max) = maybe_max { max } else { min };
        let entry = event_map
            .entry(topic.into())
            .or_insert_with(|| Vec::with_capacity(estimated_capacity));
        event_iter.for_each(|ev| entry.push(ev));
        Ok(())
    }
}

impl<E, A> EventSource<E, A> for InMemoryStore<E, A>
where
    A: WithAggregateId,
{
    type Error = String;

    fn read_events<R>(&self, topic: &str, subscriber: &mut R) -> Result<(), Self::Error>
    where
        E: EventType,
        A: WithAggregateId,
        R: ReceiveEvent<E, A>,
    {
        let event_map = self.events.read().map_err(|err| err.to_string())?;
        event_map
            .get(topic)
            .iter()
            .for_each(|events| events.iter().for_each(|ev| subscriber.receive_event(ev)));
        Ok(())
    }
}
