use crate::{DomainEvent, EventType, WithAggregateId};

pub trait DispatchEvent<E, A>
where
    E: EventType,
    A: WithAggregateId,
{
    fn dispatch(&self, stream: &str, event: DomainEvent<E, A>);
}

pub trait DispatchCommand<C, A> {
    type Context;

    fn dispatch_command(command: C, context: &Self::Context);
}
