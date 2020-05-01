use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::FromIterator;

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
pub struct NewEvent<T> {
    pub timestamp: DateTime<Utc>,
    pub payload: T,
    pub metadata: HashMap<Key, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StoredEvent<T> {
    pub sequence: Sequence,
    pub timestamp: DateTime<Utc>,
    pub payload: T,
    pub metadata: HashMap<Key, Value>,
}

impl<T> StoredEvent<T> {
    pub fn new(sequence: Sequence, timestamp: DateTime<Utc>, payload: T) -> Self {
        Self {
            sequence,
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

    pub fn sequence(&self) -> Sequence {
        self.sequence
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
