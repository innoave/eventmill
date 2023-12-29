use crate::time::{Date, DateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Display};

pub type Metadata = HashMap<Key, Value>;

pub type Key = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Boolean(bool),
    Date(Date),
    DateTime(DateTime),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Integer(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Long(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Double(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<Date> for Value {
    fn from(value: Date) -> Self {
        Value::Date(value)
    }
}

impl From<DateTime> for Value {
    fn from(value: DateTime) -> Self {
        Value::DateTime(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::List(value)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(value: HashMap<String, Value>) -> Self {
        Value::Map(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub key: Key,
    pub value: Value,
}

impl From<(Key, Value)> for Attribute {
    fn from((key, value): (Key, Value)) -> Self {
        Self { key, value }
    }
}

impl Attribute {
    pub fn into_tuple(self) -> (Key, Value) {
        (self.key, self.value)
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Value::String(value) => write!(f, "{}={}", &self.key, value),
            Value::Integer(value) => write!(f, "{}={}", &self.key, value),
            Value::Long(value) => write!(f, "{}={}", &self.key, value),
            Value::Float(value) => write!(f, "{}={}", &self.key, value),
            Value::Double(value) => write!(f, "{}={}", &self.key, value),
            Value::Boolean(value) => write!(f, "{}={}", &self.key, value),
            Value::Date(value) => write!(f, "{}={}", &self.key, value),
            Value::DateTime(value) => write!(f, "{}={}", &self.key, value),
            Value::List(value) => write!(f, "{}={:?}", &self.key, value),
            Value::Map(value) => write!(f, "{}={:?}", &self.key, value),
        }
    }
}

pub trait Attributes: IntoIterator<Item = (Key, Value)> {}

impl<T> Attributes for T where T: IntoIterator<Item = (Key, Value)> {}
