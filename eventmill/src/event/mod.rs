use crate::aggregate::{AggregateIdOf, AggregateType, Generation, WithAggregateId};
use crate::metadata::{Key, Metadata, Value};
use crate::time::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};
use std::iter::FromIterator;
use std::time::SystemTime;

pub trait EventType {
    fn event_type_version(&self) -> &str;
    fn event_type(&self) -> &str;
    fn event_source(&self) -> &str;
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Sequence(u64);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainEvent<E, A>
where
    A: WithAggregateId,
{
    pub aggregate_id: AggregateIdOf<A>,
    pub sequence: Sequence,
    #[cfg_attr(feature = "time", serde(with = "time::serde::iso8601"))]
    pub time: DateTime,
    pub data: E,
    pub metadata: Metadata,
}

/// implement PartialEq without requiring type parameter A to implement PartialEq
impl<E, A> PartialEq for DomainEvent<E, A>
where
    E: PartialEq,
    A: WithAggregateId,
{
    fn eq(&self, other: &Self) -> bool {
        self.aggregate_id.eq(&other.aggregate_id)
            && self.sequence.eq(&other.sequence)
            && self.time.eq(&other.time)
            && self.data.eq(&other.data)
            && self.metadata.eq(&other.metadata)
    }
}

/// implement Clone without requiring type parameter A to implement Clone
impl<E, A> Clone for DomainEvent<E, A>
where
    E: Clone,
    A: WithAggregateId,
{
    fn clone(&self) -> Self {
        Self {
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            time: self.time,
            data: self.data.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl<E, A> DomainEvent<E, A>
where
    A: WithAggregateId,
{
    pub fn new(
        aggregate_id: impl Into<AggregateIdOf<A>>,
        sequence: Sequence,
        time: DateTime,
        data: impl Into<E>,
    ) -> Self {
        Self {
            aggregate_id: aggregate_id.into(),
            sequence,
            time,
            data: data.into(),
            metadata: Metadata::new(),
        }
    }

    pub fn new_now(
        aggregate_id: impl Into<AggregateIdOf<A>>,
        sequence: Sequence,
        data: impl Into<E>,
    ) -> Self {
        Self::new(
            aggregate_id,
            sequence,
            DateTime::from(SystemTime::now()),
            data,
        )
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> Self
    where
        M: IntoIterator<Item = (Key, Value)>,
    {
        self.metadata = Metadata::from_iter(metadata);
        self
    }

    pub fn unwrap(self) -> (E, Metadata) {
        (self.data, self.metadata)
    }

    pub fn as_view(&self) -> DomainEventView<'_, E, A> {
        DomainEventView {
            aggregate_id: &self.aggregate_id,
            sequence: self.sequence,
            time: self.time,
            data: &self.data,
            metadata: &self.metadata,
        }
    }

    pub fn aggregate_id(&self) -> &AggregateIdOf<A> {
        &self.aggregate_id
    }

    pub fn sequence(&self) -> Sequence {
        self.sequence
    }

    pub fn time(&self) -> DateTime {
        self.time
    }

    pub fn data(&self) -> &E {
        &self.data
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl<E, A> DomainEvent<E, A>
where
    A: AggregateType + WithAggregateId,
{
    pub fn aggregate_type(&self) -> &str {
        <A as AggregateType>::aggregate_type()
    }
}

#[derive(Debug)]
pub struct DomainEventView<'a, E, A>
where
    A: WithAggregateId,
{
    pub aggregate_id: &'a AggregateIdOf<A>,
    pub sequence: Sequence,
    pub time: DateTime,
    pub data: &'a E,
    pub metadata: &'a Metadata,
}

/// implement PartialEq without requiring type parameter A to implement PartialEq
impl<E, A> PartialEq for DomainEventView<'_, E, A>
where
    E: PartialEq,
    A: WithAggregateId,
{
    fn eq(&self, other: &Self) -> bool {
        self.aggregate_id.eq(other.aggregate_id)
            && self.sequence.eq(&other.sequence)
            && self.time.eq(&other.time)
            && self.data.eq(other.data)
            && self.metadata.eq(other.metadata)
    }
}

/// implement Clone without requiring type parameter A to implement Clone
impl<E, A> Clone for DomainEventView<'_, E, A>
where
    A: WithAggregateId,
{
    fn clone(&self) -> Self {
        Self {
            aggregate_id: self.aggregate_id,
            sequence: self.sequence,
            time: self.time,
            data: self.data,
            metadata: self.metadata,
        }
    }
}

impl<'a, E, A> DomainEventView<'a, E, A>
where
    A: WithAggregateId,
{
    pub fn transmute<U, B>(self, other_data: &'a U) -> DomainEventView<'_, U, B>
    where
        B: WithAggregateId,
        &'a AggregateIdOf<A>: Into<&'a AggregateIdOf<B>>,
    {
        DomainEventView {
            aggregate_id: self.aggregate_id.into(),
            sequence: self.sequence,
            time: self.time,
            data: other_data,
            metadata: self.metadata,
        }
    }

    pub fn aggregate_id(&self) -> &AggregateIdOf<A> {
        self.aggregate_id
    }

    pub fn sequence(&self) -> Sequence {
        self.sequence
    }

    pub fn time(&self) -> DateTime {
        self.time
    }

    pub fn data(&self) -> &E {
        self.data
    }

    pub fn metadata(&self) -> &Metadata {
        self.metadata
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewEvent<E, A>
where
    A: WithAggregateId,
{
    pub aggregate_id: AggregateIdOf<A>,
    pub data: E,
}

/// implement PartialEq without requiring type parameter A to implement PartialEq
impl<E, A> PartialEq for NewEvent<E, A>
where
    E: PartialEq,
    A: WithAggregateId,
{
    fn eq(&self, other: &Self) -> bool {
        self.aggregate_id.eq(&other.aggregate_id) && self.data.eq(&other.data)
    }
}

/// implement Clone without requiring type parameter A to implement Clone
impl<E, A> Clone for NewEvent<E, A>
where
    E: Clone,
    A: WithAggregateId,
{
    fn clone(&self) -> Self {
        Self {
            aggregate_id: self.aggregate_id.clone(),
            data: self.data.clone(),
        }
    }
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
    pub fn new(aggregate_id: impl Into<AggregateIdOf<A>>, data: impl Into<E>) -> Self {
        Self {
            aggregate_id: aggregate_id.into(),
            data: data.into(),
        }
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
            time: DateTime::from(SystemTime::now()),
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
            time: DateTime::from(SystemTime::now()),
            data,
            metadata: metadata.clone(),
        })
}

#[cfg(test)]
mod tests;
