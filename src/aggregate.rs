use crate::{DomainEvent, HandleCommand, NewEvent};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Generation(u64);

impl Default for Generation {
    fn default() -> Self {
        Self(0)
    }
}

impl Generation {
    pub fn number(self) -> u64 {
        self.0
    }

    pub fn next_value(self) -> Self {
        Self(self.0 + 1)
    }
}

pub trait AggregateType {
    fn aggregate_type() -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub trait WithAggregateId {
    type Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned;

    fn aggregate_id(&self) -> &Self::Id;
}

pub type AggregateIdOf<A> = <A as WithAggregateId>::Id;

pub trait AggregateState {
    fn generation(&self) -> Generation;
}

pub trait Aggregate<E>
where
    E: 'static,
    Self: 'static + Sized + WithAggregateId,
{
    fn apply_event(&mut self, event: &DomainEvent<E, Self>);

    fn apply_all_events<'a>(&mut self, events: impl IntoIterator<Item = &'a DomainEvent<E, Self>>) {
        events.into_iter().for_each(|e| self.apply_event(e))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventSourcedAggregate<S> {
    generation: Generation,
    state: S,
}

impl<S> EventSourcedAggregate<S> {
    pub fn new(state: S) -> Self {
        Self {
            generation: Generation::default(),
            state,
        }
    }

    pub fn restore(generation_number: u64, state: S) -> Self {
        Self {
            generation: Generation(generation_number),
            state,
        }
    }

    pub fn unwrap(self) -> S {
        self.state
    }

    pub fn generation(&self) -> Generation {
        self.generation
    }
}

impl<S> AggregateState for EventSourcedAggregate<S> {
    fn generation(&self) -> Generation {
        self.generation
    }
}

impl<S> WithAggregateId for EventSourcedAggregate<S>
where
    S: WithAggregateId,
{
    type Id = <S as WithAggregateId>::Id;

    fn aggregate_id(&self) -> &Self::Id {
        self.state.aggregate_id()
    }
}

impl<E, S> Aggregate<E> for EventSourcedAggregate<S>
where
    E: 'static + Clone,
    S: Aggregate<E> + AggregateType,
{
    fn apply_event(&mut self, event: &DomainEvent<E, Self>) {
        let event = DomainEvent::new(
            event.aggregate_id.clone(),
            event.sequence,
            event.time,
            event.payload.clone(),
        );
        self.state.apply_event(&event);
        self.generation = self.generation.next_value();
    }
}

impl<C, A, S> HandleCommand<C, A> for EventSourcedAggregate<S>
where
    S: HandleCommand<C, A>,
    A: WithAggregateId,
{
    type Event = <S as HandleCommand<C, A>>::Event;
    type Error = <S as HandleCommand<C, A>>::Error;

    fn handle_command(&self, command: C) -> Result<Vec<NewEvent<Self::Event, A>>, Self::Error> {
        self.state.handle_command(command)
    }
}
