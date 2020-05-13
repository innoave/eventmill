use crate::aggregate::{AggregateIdOf, AggregateType, Generation, WithAggregateId};
use crate::metadata::{Key, Metadata, Value};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};
use std::iter::FromIterator;

pub trait EventType {
    fn event_type_version(&self) -> &str;
    fn event_type(&self) -> &str;
    fn event_source(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Sequence(u64);

impl Default for Sequence {
    fn default() -> Self {
        Self(0)
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sequence {
    pub fn number(self) -> u64 {
        self.0
    }

    pub fn next_value(&mut self) -> Self {
        self.0 = self.0.wrapping_add(1);
        Self(self.0)
    }
}

impl From<Generation> for Sequence {
    fn from(generation: Generation) -> Self {
        Self(generation.number())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainEvent<E, A>
where
    A: WithAggregateId,
{
    pub aggregate_id: AggregateIdOf<A>,
    pub sequence: Sequence,
    pub time: DateTime<Utc>,
    pub data: E,
    pub metadata: Metadata,
}

impl<E, A> DomainEvent<E, A>
where
    A: AggregateType + WithAggregateId,
{
    pub fn new(
        aggregate_id: AggregateIdOf<A>,
        sequence: Sequence,
        time: DateTime<Utc>,
        data: E,
    ) -> Self {
        Self {
            aggregate_id,
            sequence,
            time,
            data,
            metadata: Metadata::new(),
        }
    }

    pub fn new_now(aggregate_id: AggregateIdOf<A>, sequence: Sequence, data: E) -> Self {
        Self::new(aggregate_id, sequence, Utc::now(), data)
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> Self
    where
        M: IntoIterator<Item = (Key, Value)>,
    {
        self.metadata = Metadata::from_iter(metadata.into_iter());
        self
    }

    pub fn unwrap(self) -> (E, Metadata) {
        (self.data, self.metadata)
    }

    pub fn aggregate_type(&self) -> &str {
        <A as AggregateType>::aggregate_type()
    }

    pub fn aggregate_id(&self) -> &AggregateIdOf<A> {
        &self.aggregate_id
    }

    pub fn sequence(&self) -> Sequence {
        self.sequence
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn data(&self) -> &E {
        &self.data
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn transmute<U>(&self, other_data: U) -> DomainEvent<U, A> {
        DomainEvent {
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            time: self.time,
            data: other_data,
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewEvent<E, A>
where
    A: WithAggregateId,
{
    pub aggregate_id: AggregateIdOf<A>,
    pub data: E,
}

impl<E, A> From<(AggregateIdOf<A>, E)> for NewEvent<E, A>
where
    A: WithAggregateId,
{
    fn from((aggregate_id, data): (AggregateIdOf<A>, E)) -> Self {
        Self { aggregate_id, data }
    }
}

impl<E, A> NewEvent<E, A>
where
    A: WithAggregateId,
{
    pub fn new(aggregate_id: AggregateIdOf<A>, data: E) -> Self {
        Self { aggregate_id, data }
    }

    pub fn unwrap(self) -> (AggregateIdOf<A>, E) {
        (self.aggregate_id, self.data)
    }

    pub fn aggregate_id(&self) -> &AggregateIdOf<A> {
        &self.aggregate_id
    }

    pub fn data(&self) -> &E {
        &self.data
    }
}

pub fn wrap_events<'a, E, A>(
    current_sequence: &'a mut Sequence,
    events: impl IntoIterator<Item = NewEvent<E, A>> + 'a,
) -> impl Iterator<Item = DomainEvent<E, A>> + 'a
where
    A: WithAggregateId,
{
    events
        .into_iter()
        .map(move |NewEvent { aggregate_id, data }| DomainEvent {
            aggregate_id,
            sequence: current_sequence.next_value(),
            time: Utc::now(),
            data,
            metadata: Default::default(),
        })
}

pub fn wrap_events_with_metadata<'a, E, A>(
    current_sequence: &'a mut Sequence,
    metadata: &'a Metadata,
    events: impl IntoIterator<Item = NewEvent<E, A>> + 'a,
) -> impl Iterator<Item = DomainEvent<E, A>> + 'a
where
    A: WithAggregateId,
{
    events
        .into_iter()
        .map(move |NewEvent { aggregate_id, data }| DomainEvent {
            aggregate_id,
            sequence: current_sequence.next_value(),
            time: Utc::now(),
            data,
            metadata: metadata.clone(),
        })
}

#[cfg(test)]
mod tests;
