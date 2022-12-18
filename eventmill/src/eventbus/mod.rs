mod ring_buffer;

use crate::{DomainEventView, WithAggregateId};
use futures_core::Stream;
use futures_core::task::Context;
use std::pin::Pin;
use std::task::Poll;

pub trait Subscribe<'a, E, A>
where
    E: 'a,
    A: WithAggregateId,
    <A as WithAggregateId>::Id: 'a,
{
    type Stream: Stream<Item = DomainEventView<'a, E, A>>;

    fn subscribe(&mut self) -> Self::Stream;
}

pub trait Publish<'a, E, A>
where
    A: WithAggregateId,
{
    fn publish(&mut self, event: DomainEventView<'a, E, A>);
}

#[derive(Debug)]
pub struct EventStream<'a, E, A>
where
    A: WithAggregateId,
{
    buffer: Vec<DomainEventView<'a, E, A>>,
    mask: usize,
    next: usize,
    current: usize,
}

impl<'a, E, A> Stream for EventStream<'a, E, A>
where
    E: 'a,
    A: WithAggregateId,
    <A as WithAggregateId>::Id: 'a,
{
    type Item = DomainEventView<'a, E, A>;

    fn poll_next(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current == self.next {
            return Poll::Pending;
        }
        let current = self.current & self.mask;
        self.current += 1;
        match self.buffer.get(current) {
            Some(event) => Poll::Ready(Some(event.clone())),
            None => Poll::Pending,
        }
    }
}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct EventBus {}

impl<'a, E, A> Subscribe<'a, E, A> for EventBus
where
    E: 'a,
    A: WithAggregateId,
    <A as WithAggregateId>::Id: 'a,
{
    type Stream = EventStream<'a, E, A>;

    fn subscribe(&mut self) -> Self::Stream {
        unimplemented!()
    }
}

impl<'a, E, A> Publish<'a, E, A> for EventBus
where
    A: WithAggregateId,
{
    fn publish(&mut self, event: DomainEventView<'a, E, A>) {}
}
