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

    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

pub trait AggregateType {
    fn aggregate_type(&self) -> &str;
}

pub trait AggregateState {
    type Id;

    fn aggregate_id(&self) -> &Self::Id;

    fn generation(&self) -> Generation;
}

pub trait Aggregate<C>
where
    Self: Sized,
{
    type Event: 'static + EventType;

    fn handle_command(&self, command: C) -> Self::Event;

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
