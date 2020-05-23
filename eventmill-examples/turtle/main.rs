mod types;

use crate::types::{Area, Direction, PenStatus, Position};
use derive_more::From;
use eventmill::dispatch::{Core, CoreError};
use eventmill::event::DomainEventView;
use eventmill::inmemory_store::InMemoryStore;
use eventmill::{
    Aggregate, AggregateType, DispatchCommand, DomainCommand, EventType, Generation, HandleCommand,
    NewEvent,
};

const EVENT_NAMESPACE: &str = "https://github.com/innoave/eventmill/examples/turtle";

//
// Domain events
//

#[derive(Debug)]
struct TurtleMoved {
    amount: u16,
}

#[derive(Debug)]
enum TurtleTurned {
    Left,
    Right,
}

#[derive(Debug)]
enum PenSwitched {
    Up,
    Down,
}

#[derive(EventType, From, Debug)]
#[event_source(EVENT_NAMESPACE)]
#[event_type_version("V2")]
enum TurtleEvent {
    Moved(TurtleMoved),
    Turned(TurtleTurned),
    PenSwitched(PenSwitched),
}

//
// Aggregate
//

#[derive(AggregateType, Debug)]
#[id_field(id)]
#[initialize_with_defaults]
struct Turtle {
    id: u32,
    position: Position,
    direction: Direction,
    pen_status: PenStatus,
}

//
// Commands
//

#[derive(Debug)]
struct Move {
    amount: u16,
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum SwitchPen {
    Up,
    Down,
}

//
// implementations
//

impl Aggregate<TurtleMoved> for Turtle {
    fn apply_event(&mut self, event: DomainEventView<'_, TurtleMoved, Self>) {
        match self.direction {
            Direction::North => self.position.y += i32::from(event.data.amount),
            Direction::East => self.position.x += i32::from(event.data.amount),
            Direction::South => self.position.y -= i32::from(event.data.amount),
            Direction::West => self.position.y -= i32::from(event.data.amount),
        }
    }
}

impl Aggregate<TurtleTurned> for Turtle {
    fn apply_event(&mut self, event: DomainEventView<'_, TurtleTurned, Self>) {
        match self.direction {
            Direction::North => match event.data {
                TurtleTurned::Left => self.direction = Direction::West,
                TurtleTurned::Right => self.direction = Direction::East,
            },
            Direction::East => match event.data {
                TurtleTurned::Left => self.direction = Direction::North,
                TurtleTurned::Right => self.direction = Direction::South,
            },
            Direction::South => match event.data {
                TurtleTurned::Left => self.direction = Direction::East,
                TurtleTurned::Right => self.direction = Direction::West,
            },
            Direction::West => match event.data {
                TurtleTurned::Left => self.direction = Direction::South,
                TurtleTurned::Right => self.direction = Direction::North,
            },
        }
    }
}

impl Aggregate<PenSwitched> for Turtle {
    fn apply_event(&mut self, event: DomainEventView<'_, PenSwitched, Self>) {
        match event.data {
            PenSwitched::Up => self.pen_status = PenStatus::Up,
            PenSwitched::Down => self.pen_status = PenStatus::Down,
        }
    }
}

impl Aggregate<TurtleEvent> for Turtle {
    fn apply_event(&mut self, event: DomainEventView<'_, TurtleEvent, Self>) {
        match &event.data {
            TurtleEvent::Moved(moved) => self.apply_event(event.transmute(moved)),
            TurtleEvent::Turned(turned) => self.apply_event(event.transmute(turned)),
            TurtleEvent::PenSwitched(pen_switched) => {
                self.apply_event(event.transmute(pen_switched))
            }
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
enum TurtleError {
    #[error("{0} border of area reached")]
    BorderOfAreaReached(Direction),
}

impl HandleCommand<Move, Turtle> for Turtle {
    type Event = TurtleEvent;
    type Error = TurtleError;
    type Context = Area;

    fn handle_command(
        &self,
        Move { amount }: Move,
        area: &Self::Context,
    ) -> Result<Vec<NewEvent<Self::Event, Turtle>>, Self::Error> {
        // validate the command
        let new_position = match self.direction {
            Direction::North => Position {
                x: self.position.x,
                y: self.position.y + i32::from(amount),
            },
            Direction::East => Position {
                x: self.position.x + i32::from(amount),
                y: self.position.y,
            },
            Direction::South => Position {
                x: self.position.x,
                y: self.position.y - i32::from(amount),
            },
            Direction::West => Position {
                x: self.position.x - i32::from(amount),
                y: self.position.y,
            },
        };

        if area.contains(new_position) {
            Ok(vec![NewEvent {
                aggregate_id: self.id,
                data: TurtleMoved { amount }.into(),
            }])
        } else {
            Err(TurtleError::BorderOfAreaReached(self.direction))
        }
    }
}

impl HandleCommand<Turn, Turtle> for Turtle {
    type Event = TurtleEvent;
    type Error = TurtleError;
    type Context = ();

    fn handle_command(
        &self,
        command: Turn,
        _context: &Self::Context,
    ) -> Result<Vec<NewEvent<Self::Event, Turtle>>, Self::Error> {
        let turned = match command {
            Turn::Left => TurtleTurned::Left,
            Turn::Right => TurtleTurned::Right,
        };

        Ok(vec![NewEvent {
            aggregate_id: self.id,
            data: turned.into(),
        }])
    }
}

impl HandleCommand<SwitchPen, Turtle> for Turtle {
    type Event = TurtleEvent;
    type Error = TurtleError;
    type Context = ();

    fn handle_command(
        &self,
        command: SwitchPen,
        _context: &Self::Context,
    ) -> Result<Vec<NewEvent<Self::Event, Turtle>>, Self::Error> {
        let event = match command {
            SwitchPen::Up => PenSwitched::Up,
            SwitchPen::Down => PenSwitched::Down,
        };

        Ok(vec![NewEvent {
            aggregate_id: self.id,
            data: event.into(),
        }])
    }
}

fn main() {
    let turtle_id = 1;

    let event_store = InMemoryStore::default();
    let core = Core::new(event_store);

    let area = Area {
        top_left: Position { x: -149, y: 105 },
        bottom_right: Position { x: 148, y: -105 },
    };

    let move_turtle: DomainCommand<Move, Turtle> = DomainCommand {
        aggregate_id: turtle_id,
        aggregate_generation: Generation::default(),
        data: Move { amount: 21 },
    };

    let turtle = core
        .dispatch_command(move_turtle, &area)
        .expect("turtle moved");

    assert_eq!(turtle.state().position, Position { x: 0, y: 21 });
    assert_eq!(turtle.state().direction, Direction::North);
    assert_eq!(turtle.state().pen_status, PenStatus::Up);

    // once we have an aggregate we can construct new domain commands by
    // converting the tuple of a command and the current aggregate.
    let turn_right = (Turn::Right, &turtle).into();

    let turtle = core
        .dispatch_command(turn_right, &())
        .expect("turtle turned");

    assert_eq!(turtle.state().position, Position { x: 0, y: 21 });
    assert_eq!(turtle.state().direction, Direction::East);
    assert_eq!(turtle.state().pen_status, PenStatus::Up);

    let switch_pen_down = (SwitchPen::Down, &turtle).into();

    let turtle = core
        .dispatch_command(switch_pen_down, &())
        .expect("pen is switched down");

    assert_eq!(turtle.state().position, Position { x: 0, y: 21 });
    assert_eq!(turtle.state().direction, Direction::East);
    assert_eq!(turtle.state().pen_status, PenStatus::Down);

    let move_10 = (Move { amount: 10 }, &turtle).into();

    let turtle = core
        .dispatch_command(move_10, &area)
        .expect("turtle turned");

    assert_eq!(turtle.state().position, Position { x: 10, y: 21 });
    assert_eq!(turtle.state().direction, Direction::East);
    assert_eq!(turtle.state().pen_status, PenStatus::Down);

    let turn_right = (Turn::Right, &turtle).into();

    let turtle = core
        .dispatch_command(turn_right, &())
        .expect("turtle turned right again");

    assert_eq!(turtle.state().position, Position { x: 10, y: 21 });
    assert_eq!(turtle.state().direction, Direction::South);
    assert_eq!(turtle.state().pen_status, PenStatus::Down);

    // now we try to move out of the area
    let move_200 = (Move { amount: 200 }, &turtle).into();

    let error = core
        .dispatch_command(move_200, &area)
        .expect_err(&format!("turtle can not move out of the area {:?}", area));

    assert_eq!(
        error,
        CoreError::HandleCommandFailed(TurtleError::BorderOfAreaReached(Direction::South))
    );

    let switch_pen_up = (SwitchPen::Up, &turtle).into();
    let turtle = core
        .dispatch_command(switch_pen_up, &())
        .expect("pen is switched up");

    assert_eq!(turtle.state().position, Position { x: 10, y: 21 });
    assert_eq!(turtle.state().direction, Direction::South);
    assert_eq!(turtle.state().pen_status, PenStatus::Up);

    let turn_left = (Turn::Left, &turtle).into();

    let turtle = core
        .dispatch_command(turn_left, &())
        .expect("turtle turned");

    assert_eq!(turtle.state().position, Position { x: 10, y: 21 });
    assert_eq!(turtle.state().direction, Direction::East);
    assert_eq!(turtle.state().pen_status, PenStatus::Up);
}
