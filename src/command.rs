use crate::{DomainEvent, Generation, WithAggregateId};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait HandleCommand<C, A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    type Event;
    type Error;

    fn handle_command(&self, command: C) -> Result<DomainEvent<Self::Event, A>, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCommand<C, A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    _aggregate: PhantomData<A>,
    pub aggregate_id: <A as WithAggregateId>::Id,
    pub aggregate_generation: Generation,
    pub payload: C,
}

impl<C, A> DomainCommand<C, A>
where
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    pub fn new(
        aggregate_id: <A as WithAggregateId>::Id,
        aggregate_generation: Generation,
        payload: C,
    ) -> Self {
        Self {
            _aggregate: PhantomData,
            aggregate_id,
            aggregate_generation,
            payload,
        }
    }
}
