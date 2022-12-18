use crate::aggregate::{
    Aggregate, AggregateType, Generation, InitializeAggregate, VersionedAggregate, WithAggregateId,
};
use crate::command::{DomainCommand, HandleCommand};
use crate::event::{wrap_events, DomainEvent, EventType, Sequence};
use crate::store::{EventSink, EventSource};
use crate::ReceiveEvent;
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
    type Output;
    type Error: std::error::Error;

    fn dispatch_command(
        &self,
        command: C,
        context: &Self::Context,
    ) -> Result<Self::Output, Self::Error>;
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
    #[error("failed to read events from the event store: {0}")]
    ReadEventsFailed(R),
    #[error("handling of command failed: {0}")]
    HandleCommandFailed(H),
    #[error("actual aggregate version ({actual}) does not match the assumed version ({assumed})")]
    GenerationConflict {
        assumed: Generation,
        actual: Generation,
    },
}

pub type CoreDispatchError<S, C, A> = CoreError<
    <S as EventSource<
        <VersionedAggregate<A> as HandleCommand<C, VersionedAggregate<A>>>::Event,
        VersionedAggregate<A>,
    >>::Error,
    <S as EventSink<
        <VersionedAggregate<A> as HandleCommand<C, VersionedAggregate<A>>>::Event,
        VersionedAggregate<A>,
    >>::Error,
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
    <A as HandleCommand<C, A>>::Event: 'static + EventType,
    S: EventSource<
            <VersionedAggregate<A> as HandleCommand<C, VersionedAggregate<A>>>::Event,
            VersionedAggregate<A>,
        > + EventSink<
            <VersionedAggregate<A> as HandleCommand<C, VersionedAggregate<A>>>::Event,
            VersionedAggregate<A>,
        >,
{
    type Context = <A as HandleCommand<C, A>>::Context;
    type Output = VersionedAggregate<A>;
    type Error = CoreDispatchError<S, C, A>;

    fn dispatch_command(
        &self,
        DomainCommand {
            aggregate_id,
            aggregate_generation,
            data,
        }: DomainCommand<C, A>,
        context: &Self::Context,
    ) -> Result<Self::Output, Self::Error> {
        // Replay aggregate
        let mut aggregate = VersionedAggregate::initialize(aggregate_id.clone());
        self.event_store
            .read(&aggregate_id, &mut aggregate)
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
        let offset = current_sequence;

        // Handle the command
        let new_events = aggregate
            .handle_command(data, context)
            .map_err(CoreError::HandleCommandFailed)?;
        let domain_events = wrap_events(&mut current_sequence, new_events);

        // Save new events in the event store
        self.event_store
            .append_batch(domain_events)
            .map_err(CoreError::AppendEventsFailed)?;

        // Apply new domain events
        self.event_store
            .read_from_offset(&aggregate_id, offset, &mut aggregate)
            .map_err(CoreError::ReadEventsFailed)?;

        Ok(aggregate)
    }
}

#[allow(missing_debug_implementations)]
pub struct Eventmill<E, A, S> {
    core: Core<S>,
    event_handlers: Vec<Box<dyn ReceiveEvent<E, A>>>,
}

impl<E, A, S> Eventmill<E, A, S>
where
    A: WithAggregateId,
{
    pub fn new(event_store: S) -> Self {
        let core = Core::new(event_store);
        Self {
            core,
            event_handlers: Vec::new(),
        }
    }

    pub fn with_event_handler(mut self, event_handler: impl ReceiveEvent<E, A> + 'static) -> Self {
        self.event_handlers.push(Box::new(event_handler));
        self
    }
}

#[cfg(test)]
mod tests;
