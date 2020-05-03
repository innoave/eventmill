use crate::metadata::{Key, Metadata, Value};
use crate::{Generation, WithAggregateId};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
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
        Self(0)
    }
}

impl Sequence {
    pub fn number(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainEvent<E, A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    _aggregate: PhantomData<A>,
    pub aggregate_id: <A as WithAggregateId>::Id,
    pub aggregate_generation: Generation,
    pub sequence: Sequence,
    pub time: DateTime<Utc>,
    pub payload: E,
    pub metadata: Metadata,
}

impl<E, A> DomainEvent<E, A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    pub fn new(
        aggregate_id: <A as WithAggregateId>::Id,
        aggregate_generation: Generation,
        sequence: Sequence,
        time: DateTime<Utc>,
        payload: E,
    ) -> Self {
        Self {
            _aggregate: PhantomData,
            aggregate_id,
            aggregate_generation,
            sequence,
            time,
            payload,
            metadata: Metadata::new(),
        }
    }

    pub fn new_now(
        aggregate_id: <A as WithAggregateId>::Id,
        aggregate_generation: Generation,
        sequence: Sequence,
        payload: E,
    ) -> Self {
        Self {
            _aggregate: PhantomData,
            aggregate_id,
            aggregate_generation,
            sequence,
            time: Utc::now(),
            payload,
            metadata: Metadata::new(),
        }
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

    pub fn aggregate_id(&self) -> &<A as WithAggregateId>::Id {
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
pub struct NewEvent<E> {
    pub time: DateTime<Utc>,
    pub payload: E,
    pub metadata: Metadata,
}

impl<E> From<E> for NewEvent<E> {
    fn from(payload: E) -> Self {
        Self::now_with(payload)
    }
}

impl<E> NewEvent<E> {
    pub fn new(time: DateTime<Utc>, payload: E) -> Self {
        Self {
            time,
            payload,
            metadata: Metadata::new(),
        }
    }

    pub fn now_with(payload: E) -> Self {
        Self {
            time: Utc::now(),
            payload,
            metadata: Metadata::new(),
        }
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
