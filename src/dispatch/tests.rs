use super::*;
use proptest::prelude::*;

mod core {
    use super::*;
    use crate::inmemory_store::InMemoryStore;
    use crate::{AggregateIdOf, AggregateState, NewEvent};
    use std::convert::Infallible;

    #[derive(Debug, Clone, PartialEq)]
    struct Increment;

    #[derive(Debug, Clone, PartialEq)]
    struct Incremented;

    impl EventType for Incremented {
        fn event_type_version(&self) -> &str {
            "V1"
        }

        fn event_type(&self) -> &str {
            "Incremented"
        }

        fn event_source(&self) -> &str {
            "https://github.com/innoave/eventmill/tests/counter"
        }
    }

    struct Counter {
        id: i32,
        generation: Generation,
        hits: u64,
    }

    impl AggregateType for Counter {}

    impl WithAggregateId for Counter {
        type Id = i32;

        fn aggregate_id(&self) -> &Self::Id {
            &self.id
        }
    }

    impl AggregateState for Counter {
        fn generation(&self) -> Generation {
            self.generation
        }

        fn generation_mut(&mut self) -> &mut Generation {
            &mut self.generation
        }
    }

    impl Aggregate<Incremented> for Counter {
        fn apply_event(&mut self, _event: &DomainEvent<Incremented, Self>) {
            self.hits += 1;
        }
    }

    impl InitializeAggregate for Counter {
        type State = Self;

        fn initialize(aggregate_id: AggregateIdOf<Self>) -> Self::State {
            Self {
                id: aggregate_id,
                generation: Default::default(),
                hits: 0,
            }
        }
    }

    impl HandleCommand<Increment, Self> for Counter {
        type Event = Incremented;
        type Error = Infallible;
        type Context = ();

        fn handle_command(
            &self,
            _command: Increment,
            _context: &Self::Context,
        ) -> Result<Vec<NewEvent<Self::Event, Counter>>, Self::Error> {
            Ok(vec![NewEvent {
                aggregate_id: self.id,
                data: Incremented,
            }])
        }
    }

    #[test]
    fn increment_counter_with_no_events_in_store() {
        let core = Core::new(InMemoryStore::default());

        let increment = Increment;
        let increment_command = DomainCommand {
            aggregate_id: 1,
            aggregate_generation: Default::default(),
            data: increment,
        };

        let versioned_counter: VersionedAggregate<Counter> = core
            .dispatch_command(increment_command, &())
            .expect("counter incremented");

        assert_eq!(versioned_counter.state().hits, 1);
    }

    proptest! {
        #[test]
        fn increment_counter_with_some_events_in_store(
            num_events in (0..404u64),
            aggregate_id in any::<i32>()
        ) {
            let (stored_events, aggregate_generation) = {
                let mut current_sequence = Sequence::default();
                (
                    wrap_events(
                        &mut current_sequence,
                        (0..num_events).map(|_| NewEvent {
                            aggregate_id,
                            data: Incremented,
                        }),
                    ).collect::<Vec<_>>(),
                    Generation::from(current_sequence),
                )
            };

            let core = Core::new(InMemoryStore::with_events(stored_events));

            let increment = Increment;
            let increment_command = DomainCommand {
                aggregate_id,
                aggregate_generation,
                data: increment,
            };

            let versioned_counter: VersionedAggregate<Counter> = core
                .dispatch_command(increment_command, &())
                .expect("counter incremented");

            prop_assert_eq!(versioned_counter.state().hits, num_events + 1);
        }
    }
}
