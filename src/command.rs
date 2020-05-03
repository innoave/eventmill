use crate::{AggregateId, Generation, NewEvent, WithAggregateId};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait HandleCommand<C, A>
where
    A: WithAggregateId,
{
    type Event;
    type Error;

    fn handle_command(&self, command: C) -> Result<Vec<NewEvent<Self::Event, A>>, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCommand<C, A>
where
    A: WithAggregateId,
{
    _aggregate: PhantomData<A>,
    pub aggregate_id: AggregateId<A>,
    pub aggregate_generation: Generation,
    pub payload: C,
}

impl<C, A> DomainCommand<C, A>
where
    A: WithAggregateId,
{
    pub fn new(aggregate_id: AggregateId<A>, aggregate_generation: Generation, payload: C) -> Self {
        Self {
            _aggregate: PhantomData,
            aggregate_id,
            aggregate_generation,
            payload,
        }
    }
}
