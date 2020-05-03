use crate::metadata::{Key, Metadata, Value};
use crate::{AggregateId, Generation, WithAggregateId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub trait EventType {
    fn event_type_version(&self) -> &str;
    fn event_type(&self) -> &str;
    fn event_source(&self) -> &str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Sequence(u64);

impl Default for Sequence {
    fn default() -> Self {
        Self(1)
    }
}

impl Sequence {
    pub fn number(self) -> u64 {
        self.0
    }

    pub fn next_value(&mut self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainEvent<E, A>
where
    A: WithAggregateId,
{
    _aggregate_type: PhantomData<A>,
    pub aggregate_id: AggregateId<A>,
    pub aggregate_generation: Generation,
    pub sequence: Sequence,
    pub time: DateTime<Utc>,
    pub payload: E,
    pub metadata: Metadata,
}

impl<E, A> DomainEvent<E, A>
where
    A: WithAggregateId,
{
    pub fn new(
        aggregate_id: AggregateId<A>,
        aggregate_generation: Generation,
        sequence: Sequence,
        time: DateTime<Utc>,
        payload: E,
    ) -> Self {
        Self {
            _aggregate_type: PhantomData,
            aggregate_id,
            aggregate_generation,
            sequence,
            time,
            payload,
            metadata: Metadata::new(),
        }
    }

    pub fn new_now(
        aggregate_id: AggregateId<A>,
        aggregate_generation: Generation,
        sequence: Sequence,
        payload: E,
    ) -> Self {
        Self::new(
            aggregate_id,
            aggregate_generation,
            sequence,
            Utc::now(),
            payload,
        )
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> Self
    where
        M: IntoIterator<Item = (Key, Value)>,
    {
        self.metadata = Metadata::from_iter(metadata.into_iter());
        self
    }

    pub fn unwrap(self) -> (E, Metadata) {
        (self.payload, self.metadata)
    }

    pub fn aggregate_id(&self) -> &AggregateId<A> {
        &self.aggregate_id
    }

    pub fn aggregate_generation(&self) -> Generation {
        self.aggregate_generation
    }

    pub fn sequence(&self) -> Sequence {
        self.sequence
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn payload(&self) -> &E {
        &self.payload
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewEvent<E, A>
where
    A: WithAggregateId,
{
    _aggregate_type: PhantomData<A>,
    pub aggregate_id: AggregateId<A>,
    pub time: DateTime<Utc>,
    pub payload: E,
    pub metadata: Metadata,
}

impl<E, A> From<(AggregateId<A>, E)> for NewEvent<E, A>
where
    A: WithAggregateId,
{
    fn from((aggregate_id, payload): (AggregateId<A>, E)) -> Self {
        Self::new_now(aggregate_id, payload)
    }
}

impl<E, A> NewEvent<E, A>
where
    A: WithAggregateId,
{
    pub fn new(aggregate_id: AggregateId<A>, time: DateTime<Utc>, payload: E) -> Self {
        Self {
            _aggregate_type: PhantomData,
            aggregate_id,
            time,
            payload,
            metadata: Metadata::new(),
        }
    }

    pub fn new_now(aggregate_id: AggregateId<A>, payload: E) -> Self {
        Self::new(aggregate_id, Utc::now(), payload)
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> Self
    where
        M: IntoIterator<Item = (Key, Value)>,
    {
        self.metadata = Metadata::from_iter(metadata.into_iter());
        self
    }

    pub fn unwrap(self) -> (E, Metadata) {
        (self.payload, self.metadata)
    }

    pub fn aggregate_id(&self) -> &AggregateId<A> {
        &self.aggregate_id
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }

    pub fn payload(&self) -> &E {
        &self.payload
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
