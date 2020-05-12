use crate::aggregate::{AggregateIdOf, Generation, WithAggregateId};
use crate::event::NewEvent;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type EventOf<H, C, A> = <H as HandleCommand<C, A>>::Event;

pub trait HandleCommand<C, A>
where
    A: WithAggregateId,
{
    type Event;
    type Error: std::error::Error;
    type Context;

    fn handle_command(
        &self,
        command: C,
        context: &Self::Context,
    ) -> Result<Vec<NewEvent<Self::Event, A>>, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainCommand<C, A>
where
    A: WithAggregateId,
{
    pub aggregate_id: AggregateIdOf<A>,
    pub aggregate_generation: Generation,
    pub data: C,
}

impl<C, A> DomainCommand<C, A>
where
    A: WithAggregateId,
{
    pub fn new(aggregate_id: AggregateIdOf<A>, aggregate_generation: Generation, data: C) -> Self {
        Self {
            aggregate_id,
            aggregate_generation,
            data,
        }
    }
}
