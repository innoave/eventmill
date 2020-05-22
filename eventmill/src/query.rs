use crate::aggregate::{Aggregate, WithAggregateId};
use crate::event::DomainEventView;

pub trait ReceiveEvent<E, A>
where
    A: WithAggregateId,
{
    fn receive_event(&mut self, event: DomainEventView<'_, E, A>);
}

impl<E, A> ReceiveEvent<E, Self> for A
where
    E: 'static,
    A: Aggregate<E> + WithAggregateId,
{
    fn receive_event(&mut self, event: DomainEventView<'_, E, Self>) {
        self.apply_event(event);
    }
}
