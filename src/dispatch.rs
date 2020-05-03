use crate::{DomainEvent, EventType, WithAggregateId};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait Dispatch {
    fn dispatch<E, A>(&self, event: DomainEvent<E, A>, stream: &str)
    where
        E: EventType,
        A: WithAggregateId,
        <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned;
}
