use super::*;
use proptest::prelude::*;

mod sequence {
    use super::*;

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

    #[test]
    fn can_be_converted_from_a_generation() {
        let generation = Generation::default();

        let sequence = Sequence::from(generation);

        assert_eq!(sequence.number(), generation.number());
    }
}
