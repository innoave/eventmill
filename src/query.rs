use crate::{Aggregate, DomainEvent, WithAggregateId};

pub trait ReceiveEvent<E, A>
where
    A: WithAggregateId,
{
    fn receive_event(&mut self, event: &DomainEvent<E, A>);
}

impl<E, A> ReceiveEvent<E, Self> for A
where
    E: 'static,
    A: Aggregate<E> + WithAggregateId,
{
    fn receive_event(&mut self, event: &DomainEvent<E, Self>) {
        self.apply_event(event);
    }
}
