use eventmill::dispatch::Core;
use eventmill::event::{wrap_events, DomainEventView};
use eventmill::inmemory_store::InMemoryStore;
use eventmill::{
    Aggregate, AggregateType, DispatchCommand, DomainCommand, EventType, Generation, HandleCommand,
    NewEvent, Sequence, VersionedAggregate,
};
use std::convert::Infallible;

const EVENT_NAMESPACE: &str = "https://github.com/innoave/eventmill/examples/counter";

//
// Domain events
//

#[derive(EventType, Debug, Clone, PartialEq)]
#[event_source(EVENT_NAMESPACE)]
#[event_type_version("V1")]
#[event_type("Incremented")]
struct Incremented;

//
// Aggregate
//

#[derive(AggregateType, Debug)]
#[id_field(id)]
#[initialize_with_defaults]
struct Counter {
    id: i32,
    hits: u64,
}

impl Aggregate<Incremented> for Counter {
    fn apply_event(&mut self, _event: DomainEventView<'_, Incremented, Self>) {
        self.hits += 1;
    }
}

//
// Commands
//

#[derive(Debug, PartialEq)]
struct Increment;

impl HandleCommand<Increment, Counter> for Counter {
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

fn main() {
    let aggregate_id = 4711;

    println!(
        "prepare eventstore with 2 `Incremented` events for aggregate id {}",
        aggregate_id
    );

    let (stored_events, aggregate_generation) = {
        let mut current_sequence = Sequence::default();
        (
            wrap_events(
                &mut current_sequence,
                vec![
                    NewEvent {
                        aggregate_id: 4711,
                        data: Incremented,
                    },
                    NewEvent {
                        aggregate_id: 4711,
                        data: Incremented,
                    },
                ],
            )
            .collect::<Vec<_>>(),
            Generation::from(current_sequence),
        )
    };

    let event_store = InMemoryStore::with_events(stored_events);

    println!("|-> {:#?}", event_store);

    println!("prepare `Increment` command");

    let increment_command = DomainCommand {
        aggregate_id,
        aggregate_generation,
        data: Increment,
    };

    println!("|-> {:#?}", increment_command);

    println!("setup `Core` command dispatcher and process the command");

    let core = Core::new(event_store);

    let versioned_counter: VersionedAggregate<Counter> = core
        .dispatch_command(increment_command, &())
        .expect("counter incremented");

    println!("final result: {:#?}", versioned_counter.state());

    assert_eq!(versioned_counter.state().hits, 3);
}
