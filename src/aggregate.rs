use crate::{DomainEvent, HandleCommand};
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

    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

pub trait AggregateType {
    fn aggregate_type(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

pub trait WithAggregateId {
    type Id;

    fn aggregate_id(&self) -> &Self::Id;
}

pub trait AggregateState {
    fn generation(&self) -> Generation;
}

pub trait Aggregate<E>
where
    E: 'static,
    Self: 'static + Sized + WithAggregateId,
    <Self as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    fn apply_event(self, event: &DomainEvent<E, Self>) -> Self;

    fn apply_all_events<'a>(
        self,
        events: impl IntoIterator<Item = &'a DomainEvent<E, Self>>,
    ) -> Self {
        events
            .into_iter()
            .fold(self, |acc_state, event| acc_state.apply_event(event))
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
    S: Aggregate<E>,
    <Self as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    fn apply_event(self, event: &DomainEvent<E, Self>) -> Self {
        let event = DomainEvent::new(
            event.aggregate_id.clone(),
            event.aggregate_generation,
            event.sequence,
            event.timestamp,
            event.payload.clone(),
        );
        Self {
            generation: self.generation.next(),
            state: self.state.apply_event(&event),
        }
    }
}

impl<C, A, S> HandleCommand<C, A> for EventSourcedAggregate<S>
where
    S: HandleCommand<C, A>,
    A: WithAggregateId,
    <A as WithAggregateId>::Id: Debug + Clone + PartialEq + Serialize + DeserializeOwned,
{
    type Event = <S as HandleCommand<C, A>>::Event;
    type Error = <S as HandleCommand<C, A>>::Error;

    fn handle_command(&self, command: C) -> Result<DomainEvent<Self::Event, A>, Self::Error> {
        self.state.handle_command(command)
    }
}
