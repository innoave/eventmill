use super::*;
use proptest::prelude::*;

mod sequence {
    use super::*;
    use std::u64;

    #[test]
    fn default_sequence_number_is_0() {
        let sequence = Sequence::default();

        assert_eq!(sequence.number(), 0);
    }

    proptest! {
        #[test]
        fn next_value_returns_this_sequence_plus_1(
            number in (0..u64::MAX)
        ) {
            let mut sequence = Sequence(number);

            let next_value = sequence.next_value();

            prop_assert_eq!(next_value.number(), number + 1);
        }
    }

    #[test]
    fn next_value_wraps_around_on_max_value() {
        let mut sequence = Sequence(u64::MAX);

        let next_value = sequence.next_value();

        assert_eq!(next_value.number(), 0);
    }

    proptest! {
        #[test]
        fn display_formats_like_an_integer(
            number in (0..=u64::MAX)
        ) {
            let sequence = Sequence(number);

            prop_assert_eq!(format!("{}", sequence), format!("{}", number));
            prop_assert_eq!(sequence.to_string(), number.to_string());
        }
    }

    proptest! {
        #[test]
        fn can_be_converted_into_a_generation(
            number in (0..=u64::MAX)
        ) {
            let sequence = Sequence(number);

            let generation: Generation = sequence.into();

            assert_eq!(generation.number(), sequence.number());
        }
    }

    #[test]
    fn can_be_converted_from_a_generation() {
        let generation = Generation::default();

        let sequence = Sequence::from(generation);

        assert_eq!(sequence.number(), generation.number());
    }
}

mod domain_event {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Moved {
        velocity: i32,
    }

    #[derive(Debug)]
    struct Turtle {
        id: u32,
        pos_x: i32,
        pos_y: i32,
    }

    impl WithAggregateId for Turtle {
        type Id = u32;

        fn aggregate_id(&self) -> &Self::Id {
            &self.id
        }
    }

    proptest! {
        #[test]
        fn two_events_are_equal_if_all_fields_are_equal(
            velocity in any::<i32>(),
            aggregate_id in any::<u32>(),
            sequence in any::<u64>(),
        ) {
            let time = Utc::now();

            let event1: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id,
                sequence: Sequence(sequence),
                time,
                data: Moved { velocity },
                metadata: Metadata::default(),
            };

            let event2: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id,
                sequence: Sequence(sequence),
                time,
                data: Moved { velocity },
                metadata: Metadata::default(),
            };

            prop_assert_eq!(event1, event2);
        }

        #[test]
        fn two_events_are_not_equal_if_the_data_fields_are_different(
            (velocity1, velocity2) in (any::<i32>(), any::<i32>()).prop_filter(
                "any two velocities that are not equal",
                |(velo1, velo2)| velo1 != velo2,
            ),
            aggregate_id in any::<u32>(),
            sequence in any::<u64>(),
        ) {
            let time = Utc::now();

            let event1: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id,
                sequence: Sequence(sequence),
                time,
                data: Moved { velocity: velocity1 },
                metadata: Metadata::default(),
            };

            let event2: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id,
                sequence: Sequence(sequence),
                time,
                data: Moved { velocity: velocity2 },
                metadata: Metadata::default(),
            };

            prop_assert_ne!(event1, event2);
        }

        #[test]
        fn two_events_are_not_equal_if_the_id_fields_are_different(
            velocity in any::<i32>(),
            (aggregate_id1, aggregate_id2) in (any::<u32>(), any::<u32>()).prop_filter(
                "any two aggregate ids that are not equal",
                |(id1, id2)| id1 != id2,
            ),
            sequence in any::<u64>(),
        ) {
            let time = Utc::now();

            let event1: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id: aggregate_id1,
                sequence: Sequence(sequence),
                time,
                data: Moved { velocity },
                metadata: Metadata::default(),
            };

            let event2: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id: aggregate_id2,
                sequence: Sequence(sequence),
                time,
                data: Moved { velocity },
                metadata: Metadata::default(),
            };

            prop_assert_ne!(event1, event2);
        }

        #[test]
        fn two_events_are_not_equal_if_the_sequence_fields_are_different(
            velocity in any::<i32>(),
            aggregate_id in any::<u32>(),
            (sequence1, sequence2) in (any::<u64>(), any::<u64>()).prop_filter(
                "any two sequence numbers that are not equal",
                |(seq1, seq2)| seq1 != seq2,
            ),
        ) {
            let time = Utc::now();

            let event1: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id,
                sequence: Sequence(sequence1),
                time,
                data: Moved { velocity },
                metadata: Metadata::default(),
            };

            let event2: DomainEvent<_, Turtle> = DomainEvent {
                aggregate_id,
                sequence: Sequence(sequence2),
                time,
                data: Moved { velocity },
                metadata: Metadata::default(),
            };

            prop_assert_ne!(event1, event2);
        }
    }
}

mod new_event {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Moved {
        velocity: i32,
    }

    #[derive(Debug)]
    struct Turtle {
        id: u32,
        pos_x: i32,
        pos_y: i32,
    }

    impl WithAggregateId for Turtle {
        type Id = u32;

        fn aggregate_id(&self) -> &Self::Id {
            &self.id
        }
    }

    proptest! {
        #[test]
        fn two_events_are_equal_if_all_fields_are_equal(
            velocity in any::<i32>(),
            aggregate_id in any::<u32>(),
        ) {
            let event1: NewEvent<_, Turtle> = NewEvent {
                aggregate_id,
                data: Moved { velocity },
            };

            let event2: NewEvent<_, Turtle> = NewEvent {
                aggregate_id,
                data: Moved { velocity },
            };

            prop_assert_eq!(event1, event2);
        }

        #[test]
        fn two_events_are_not_equal_if_the_data_fields_are_different(
            (velocity1, velocity2) in (any::<i32>(), any::<i32>()).prop_filter(
                "any two velocities that are not equal",
                |(velo1, velo2)| velo1 != velo2,
            ),
            aggregate_id in any::<u32>(),
        ) {
            let event1: NewEvent<_, Turtle> = NewEvent {
                aggregate_id,
                data: Moved { velocity: velocity1 },
            };

            let event2: NewEvent<_, Turtle> = NewEvent {
                aggregate_id,
                data: Moved { velocity: velocity2 },
            };

            prop_assert_ne!(event1, event2);
        }

        #[test]
        fn two_events_are_not_equal_if_the_id_fields_are_different(
            velocity in any::<i32>(),
            (aggregate_id1, aggregate_id2) in (any::<u32>(), any::<u32>()).prop_filter(
                "any two aggregate ids that are not equal",
                |(id1, id2)| id1 != id2,
            ),
        ) {
            let event1: NewEvent<_, Turtle> = NewEvent {
                aggregate_id: aggregate_id1,
                data: Moved { velocity },
            };

            let event2: NewEvent<_, Turtle> = NewEvent {
                aggregate_id: aggregate_id2,
                data: Moved { velocity },
            };

            prop_assert_ne!(event1, event2);
        }
    }
}

mod wrap_events {
    use super::*;
    use chrono::Utc;
    use std::u64;

    #[derive(Debug, PartialEq)]
    enum TestEvent {
        Counted(usize),
    }

    struct TestAggregate {
        id: String,
        _state: String,
    }

    impl WithAggregateId for TestAggregate {
        type Id = String;

        fn aggregate_id(&self) -> &Self::Id {
            &self.id
        }
    }

    proptest! {
        #[test]
        fn number_of_returned_events_is_equal_to_the_number_of_input_events(
            number_of_events in (0..=404_usize)
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();

            let domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            prop_assert_eq!(domain_events.len(), number_of_events);
        }

        #[test]
        fn the_order_of_the_events_does_not_change(
            number_of_events in (0..=404_usize)
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();

            let domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            (1..=number_of_events).zip(domain_events.iter()).for_each(|(n, ev)| {
                assert_eq!(ev.data, TestEvent::Counted(n));
            });
        }

        #[test]
        fn the_returned_events_have_sequence_numbers_in_ascending_order(
            number_of_events in (0..=404_usize),
            initial_sequence in (0..u64::MAX / 2),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence(initial_sequence);

            let domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            (1..=number_of_events as u64).zip(domain_events.iter()).for_each(|(n, ev)| {
                assert_eq!(ev.sequence.number(), initial_sequence + n);
            });
        }

        #[test]
        fn current_sequence_increases_by_the_number_of_events(
            number_of_events in (0..=404_usize),
            initial_sequence in (0..u64::MAX / 2),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence(initial_sequence);

            let _domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            prop_assert_eq!(current_sequence.number(), initial_sequence + number_of_events as u64);
        }

        #[test]
        fn the_time_of_the_events_is_between_start_and_end_of_the_wrap_function(
            number_of_events in (0..=404_usize),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();

            let start = Utc::now();
            let domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();
            let end = Utc::now();

            domain_events.iter().for_each(|ev| {
                assert!(ev.time >= start, "event.time is not greater or equal start time");
                assert!(ev.time <= end, "event.time is not less or equal end time");
            });
        }

        #[test]
        fn the_time_of_the_events_is_in_ascending_order(
            number_of_events in (0..=404_usize),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();

            let domain_events = wrap_events(&mut current_sequence, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            domain_events.iter().zip(domain_events.iter().skip(1)).for_each(|(ev1, ev2)| {
                assert!(ev1.time <= ev2.time, "a previous event does not have a time less or equal to the subsequent event");
            });
        }
    }
}

mod wrap_events_with_metadata {
    use super::*;
    use chrono::Utc;
    use std::u64;

    #[derive(Debug, PartialEq)]
    enum TestEvent {
        Counted(usize),
    }

    struct TestAggregate {
        id: String,
        _state: String,
    }

    impl WithAggregateId for TestAggregate {
        type Id = String;

        fn aggregate_id(&self) -> &Self::Id {
            &self.id
        }
    }

    proptest! {
        #[test]
        fn number_of_returned_events_is_equal_to_the_number_of_input_events(
            number_of_events in (0..=404_usize)
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            prop_assert_eq!(domain_events.len(), number_of_events);
        }

        #[test]
        fn the_order_of_the_events_does_not_change(
            number_of_events in (0..=404_usize)
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            (1..=number_of_events).zip(domain_events.iter()).for_each(|(n, ev)| {
                assert_eq!(ev.data, TestEvent::Counted(n));
            });
        }

        #[test]
        fn the_returned_events_have_sequence_numbers_in_ascending_order(
            number_of_events in (0..=404_usize),
            initial_sequence in (0..u64::MAX / 2),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence(initial_sequence);
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            (1..=number_of_events as u64).zip(domain_events.iter()).for_each(|(n, ev)| {
                assert_eq!(ev.sequence.number(), initial_sequence + n);
            });
        }

        #[test]
        fn current_sequence_increases_by_the_number_of_events(
            number_of_events in (0..=404_usize),
            initial_sequence in (0..u64::MAX / 2),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence(initial_sequence);
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let _domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            prop_assert_eq!(current_sequence.number(), initial_sequence + number_of_events as u64);
        }

        #[test]
        fn the_time_of_the_events_is_between_start_and_end_of_the_wrap_function(
            number_of_events in (0..=404_usize),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let start = Utc::now();
            let domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();
            let end = Utc::now();

            domain_events.iter().for_each(|ev| {
                assert!(ev.time >= start, "event.time is not greater or equal start time");
                assert!(ev.time <= end, "event.time is not less or equal end time");
            });
        }

        #[test]
        fn the_time_of_the_events_is_in_ascending_order(
            number_of_events in (0..=404_usize),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            domain_events.iter().zip(domain_events.iter().skip(1)).for_each(|(ev1, ev2)| {
                assert!(ev1.time <= ev2.time, "a previous event does not have a time less or equal to the subsequent event");
            });
        }

        #[test]
        fn all_events_get_a_copy_of_the_same_metadata(
            number_of_events in (0..=404_usize),
        ) {
            let aggregate = TestAggregate { id: "0815".to_string(), _state: "42".to_string() };

            let new_events = (1..=number_of_events).map(|n|
                NewEvent {
                    aggregate_id: aggregate.id.clone(),
                    data: TestEvent::Counted(n)
                }
            );

            let mut current_sequence = Sequence::default();
            let metadata = Metadata::from_iter(vec![
                ("username".to_string(), Value::String("jane.doe".to_string())),
                ("agent".to_string(), Value::String("mozilla".to_string())),
            ].into_iter());

            let domain_events = wrap_events_with_metadata(&mut current_sequence, &metadata, new_events).collect::<Vec<DomainEvent<_, TestAggregate>>>();

            domain_events.iter().for_each(|ev| {
                assert_eq!(ev.metadata, metadata);
            });
        }
    }
}
