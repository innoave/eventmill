use crate::metadata::{Key, Metadata, Value};
use crate::{AggregateIdOf, AggregateType, WithAggregateId};
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
    pub aggregate_id: AggregateIdOf<A>,
    pub sequence: Sequence,
    pub time: DateTime<Utc>,
    pub payload: E,
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
        payload: E,
    ) -> Self {
        Self {
            _aggregate_type: PhantomData,
            aggregate_id,
            sequence,
            time,
            payload,
            metadata: Metadata::new(),
        }
    }

    pub fn new_now(aggregate_id: AggregateIdOf<A>, sequence: Sequence, payload: E) -> Self {
        Self::new(aggregate_id, sequence, Utc::now(), payload)
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

    pub fn payload(&self) -> &E {
        &self.payload
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn transmute<U>(&self, another_payload: U) -> DomainEvent<U, A> {
        DomainEvent {
            _aggregate_type: self._aggregate_type,
            aggregate_id: self.aggregate_id.clone(),
            sequence: self.sequence,
            time: self.time,
            payload: another_payload,
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
    pub payload: E,
}

impl<E, A> From<(AggregateIdOf<A>, E)> for NewEvent<E, A>
where
    A: WithAggregateId,
{
    fn from((aggregate_id, payload): (AggregateIdOf<A>, E)) -> Self {
        Self::new(aggregate_id, payload)
    }
}

impl<E, A> NewEvent<E, A>
where
    A: WithAggregateId,
{
    pub fn new(aggregate_id: AggregateIdOf<A>, payload: E) -> Self {
        Self {
            aggregate_id,
            payload,
        }
    }

    pub fn unwrap(self) -> (AggregateIdOf<A>, E) {
        (self.aggregate_id, self.payload)
    }

    pub fn aggregate_id(&self) -> &AggregateIdOf<A> {
        &self.aggregate_id
    }

    pub fn payload(&self) -> &E {
        &self.payload
    }
}
