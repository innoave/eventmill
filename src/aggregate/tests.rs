use super::*;
use proptest::prelude::*;

mod generation {
    use super::*;
    use crate::event::Sequence;
    use std::u64;

    #[test]
    fn default_generation_number_is_0() {
        let generation = Generation::default();

        assert_eq!(generation.number(), 0);
    }

    proptest! {
        #[test]
        fn increment_increases_the_number_by_1(
            number in (0..u64::MAX)
        ) {
            let mut generation = Generation(number);

            generation.increment();

            prop_assert_eq!(generation.number(), number + 1);
        }
    }

    #[test]
    fn increment_wraps_around_on_max_value() {
        let mut generation = Generation(u64::MAX);

        generation.increment();

        assert_eq!(generation.number(), 0);
    }

    proptest! {
        #[test]
        fn display_formats_like_an_integer(
            number in (0..=u64::MAX)
        ) {
            let generation = Generation(number);

            prop_assert_eq!(format!("{}", generation), format!("{}", number));
            prop_assert_eq!(generation.to_string(), number.to_string());
        }
    }

    proptest! {
        #[test]
        fn can_be_converted_into_a_sequence(
            number in (0..=u64::MAX)
        ) {
            let generation = Generation(number);

            let sequence: Sequence = generation.into();

            assert_eq!(sequence.number(), generation.number());
        }
    }

    #[test]
    fn can_be_converted_from_a_sequence() {
        let sequence = Sequence::default();

        let generation = Generation::from(sequence);

        assert_eq!(generation.number(), sequence.number());
    }
}
