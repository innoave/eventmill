#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
//    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::FromIterator;

pub trait EventType {
    fn event_type_version(&self) -> &str;
    fn event_type(&self) -> &str;
    fn event_source(&self) -> &str;
}

pub trait AggregateState {
    fn generation(&self) -> u64;
}

pub trait Aggregate<C>
where
    Self: Sized,
{
    type Event: 'static + EventType;

    fn handle_command(&self, command: C) -> Self::Event;

    fn apply_event(self, event: &EsEvent<Self::Event>) -> Self;

    fn apply_all_events<'a>(
        self,
        events: impl IntoIterator<Item = &'a EsEvent<Self::Event>>,
    ) -> Self {
        events
            .into_iter()
            .fold(self, |acc_state, event| acc_state.apply_event(event))
    }
}

pub trait EventStore {
    type Error;

    fn append<E>(&mut self, event: E, stream: &str) -> Result<(), Self::Error>
    where
        E: EventType;
}

pub trait Dispatch {
    fn dispatch<E>(&self, event: E, stream: &str)
    where
        E: EventType;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub key: Key,
    pub value: Value,
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
pub struct EsEvent<T> {
    pub sequence_number: u64,
    pub timestamp: DateTime<Utc>,
    pub payload: T,
    pub metadata: HashMap<Key, Value>,
}

impl<T> EsEvent<T> {
    pub fn new(sequence_number: u64, timestamp: DateTime<Utc>, payload: T) -> Self {
        Self {
            sequence_number,
            timestamp,
            payload,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata<M>(mut self, metadata: M) -> Self
    where
        M: IntoIterator<Item = (Key, Value)>,
    {
        self.metadata = HashMap::from_iter(metadata.into_iter());
        self
    }

    pub fn unwrap(self) -> (T, HashMap<Key, Value>) {
        (self.payload, self.metadata)
    }

    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn metadata(&self) -> &HashMap<Key, Value> {
        &self.metadata
    }
}
