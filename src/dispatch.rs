use crate::aggregate::{
    Aggregate, AggregateType, Generation, InitializeAggregate, VersionedAggregate, WithAggregateId,
};
use crate::command::{DomainCommand, HandleCommand};
use crate::event::{wrap_events, DomainEvent, EventType, Sequence};
use crate::store::{EventSink, EventSource};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

pub trait DispatchEvent<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    fn dispatch(&self, event: DomainEvent<E, A>);
}

pub trait DispatchCommand<C, A> {
    type Context;
    type Error: std::error::Error;

    fn dispatch_command(
        &self,
        command: C,
        context: &Self::Context,
    ) -> Result<VersionedAggregate<A>, Self::Error>;
}

#[derive(thiserror::Error, Debug, PartialEq, Serialize, Deserialize)]
pub enum CoreError<R, W, H>
where
    R: Debug + Display,
    W: Debug + Display,
    H: Debug + Display,
{
    #[error("failed to replay aggregate from event store: {0}")]
    ReplayAggregateFailed(R),
    #[error("failed to append events to the event store: {0}")]
    AppendEventsFailed(W),
    #[error("handling of command failed: {0}")]
    HandleCommandFailed(H),
    #[error("actual aggregate version ({actual}) does not match the assumed version ({assumed})")]
    GenerationConflict {
        assumed: Generation,
        actual: Generation,
    },
}

pub type CoreDispatchError<S, C, A> = CoreError<
    <S as EventSource<<A as HandleCommand<C, A>>::Event, VersionedAggregate<A>>>::Error,
    <S as EventSink<<A as HandleCommand<C, A>>::Event, VersionedAggregate<A>>>::Error,
    <A as HandleCommand<C, A>>::Error,
>;

#[derive(Debug)]
pub struct Core<S> {
    event_store: S,
}

impl<S> Core<S> {
    pub fn new(event_store: S) -> Self {
        Self { event_store }
    }
}

impl<C, A, S> DispatchCommand<DomainCommand<C, A>, A> for Core<S>
where
    A: Aggregate<<A as HandleCommand<C, A>>::Event>
        + AggregateType
        + WithAggregateId
        + HandleCommand<C, A>
        + InitializeAggregate<State = A>,
    <A as HandleCommand<C, A>>::Event: 'static + EventType + Clone,
    S: EventSource<<A as HandleCommand<C, A>>::Event, VersionedAggregate<A>>
        + EventSink<<A as HandleCommand<C, A>>::Event, VersionedAggregate<A>>,
{
    type Context = <A as HandleCommand<C, A>>::Context;
    type Error = CoreDispatchError<S, C, A>;

    fn dispatch_command(
        &self,
        DomainCommand {
            aggregate_id,
            aggregate_generation,
            data,
        }: DomainCommand<C, A>,
        context: &Self::Context,
    ) -> Result<VersionedAggregate<A>, Self::Error> {
        // Replay aggregate
        let mut aggregate = VersionedAggregate::initialize(aggregate_id.clone());
        self.event_store
            .read_events(&aggregate_id, &mut aggregate)
            .map_err(CoreError::ReplayAggregateFailed)?;

        // Check for generation conflict, which means whether the command was
        // issued based on the current state of the aggregate
        if aggregate_generation != aggregate.generation() {
            return Err(CoreError::GenerationConflict {
                assumed: aggregate_generation,
                actual: aggregate.generation(),
            });
        }

        let mut current_sequence = Sequence::from(aggregate.generation());

        // Handle the command
        let new_events = aggregate
            .handle_command(data, context)
            .map_err(CoreError::HandleCommandFailed)?;
        let domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<_>>();

        // Apply new domain events
        aggregate.apply_all_events(&domain_events);

        self.event_store
            .append_batch(domain_events)
            .map_err(CoreError::AppendEventsFailed)?;

        Ok(aggregate)
    }
}
