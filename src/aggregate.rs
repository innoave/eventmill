use crate::event::{EventType, StoredEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

pub trait Aggregate<C>
where
    Self: Sized,
{
    type Event: 'static + EventType;
    type Error;

    fn handle_command(&self, command: C) -> Result<Self::Event, Self::Error>;

    fn apply_event(self, event: &StoredEvent<Self::Event>) -> Self;

    fn apply_all_events<'a>(
        self,
        events: impl IntoIterator<Item = &'a StoredEvent<Self::Event>>,
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

impl<C, S> Aggregate<C> for EventSourcedAggregate<S>
where
    S: Aggregate<C>,
{
    type Event = <S as Aggregate<C>>::Event;
    type Error = <S as Aggregate<C>>::Error;

    fn handle_command(&self, command: C) -> Result<Self::Event, Self::Error> {
        self.state.handle_command(command)
    }

    fn apply_event(self, event: &StoredEvent<Self::Event>) -> Self {
        Self {
            generation: self.generation.next(),
            state: self.state.apply_event(event),
        }
    }
}
