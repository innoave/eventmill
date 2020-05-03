use crate::{DomainEvent, EventType, WithAggregateId};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub trait EventStore<A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    type Error;

    fn append<E>(&mut self, event: DomainEvent<E, A>, stream: &str) -> Result<(), Self::Error>
    where
        E: EventType;
}
