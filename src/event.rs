use crate::{Generation, WithAggregateId};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::marker::PhantomData;

pub trait EventType {
    fn event_type_version(&self) -> &str;
    fn event_type(&self) -> &str;
    fn event_source(&self) -> &str;
}

pub type Key = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub key: Key,
    pub value: Value,
}

pub type Metadata = HashMap<Key, Value>;

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
    pub payload: E,
    pub metadata: Metadata,
}

impl<E, A> DomainEvent<E, A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    pub fn new<M>(
        aggregate_id: <A as WithAggregateId>::Id,
        aggregate_generation: Generation,
        payload: E,
        metadata: M,
    ) -> Self
    where
        M: IntoIterator<Item = (Key, Value)>,
    {
        Self {
            _aggregate: PhantomData,
            aggregate_id,
            aggregate_generation,
            payload,
            metadata: Metadata::from_iter(metadata.into_iter()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewEvent<T> {
    pub timestamp: DateTime<Utc>,
    pub payload: T,
    pub metadata: Metadata,
}

impl<T> From<T> for NewEvent<T> {
    fn from(payload: T) -> Self {
        Self::now_with(payload)
    }
}

impl<T> NewEvent<T> {
    pub fn new(timestamp: DateTime<Utc>, payload: T) -> Self {
        Self {
            timestamp,
            payload,
            metadata: Metadata::new(),
        }
    }

    pub fn now_with(payload: T) -> Self {
        Self {
            timestamp: Utc::now(),
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

    pub fn unwrap(self) -> (T, Metadata) {
        (self.payload, self.metadata)
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredEvent<T> {
    pub sequence: Sequence,
    pub timestamp: DateTime<Utc>,
    pub payload: T,
    pub metadata: Metadata,
}

impl<T> StoredEvent<T> {
    pub fn new(sequence: Sequence, timestamp: DateTime<Utc>, payload: T) -> Self {
        Self {
            sequence,
            timestamp,
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

    pub fn unwrap(self) -> (T, Metadata) {
        (self.payload, self.metadata)
    }

    pub fn sequence(&self) -> Sequence {
        self.sequence
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
