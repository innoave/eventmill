use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Sequence(usize);

impl Default for Sequence {
    fn default() -> Self {
        Sequence(0)
    }
}

impl Sequence {
    pub const fn min_value() -> Self {
        Sequence(usize::min_value())
    }

    pub const fn max_value() -> Self {
        Sequence(usize::max_value())
    }

    pub fn val(self) -> usize {
        self.0
    }

    pub fn next(&mut self) -> Self {
        let current = self.0;
        self.0 += 1;
        Sequence(current)
    }
}

#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    index_mask: usize,
    read_cursor: Sequence,
    write_cursor: Sequence,
}

impl<T> RingBuffer<T> {
    pub fn with_size(size: usize) -> Self {
        let next_power_of_two = round_up_to_next_power_of_two(size);
        assert_eq!(size, next_power_of_two, "size must be a power of 2");
        let index_mask = size - 1;
        Self {
            buffer: Vec::with_capacity(size),
            index_mask,
            read_cursor: Sequence::min_value(),
            write_cursor: Sequence::min_value(),
        }
    }

    pub fn size(&self) -> usize {
        self.index_mask + 1
    }

    pub fn get(&self, sequence: Sequence) -> Option<&T> {
        if self.read_cursor >= sequence && sequence < self.write_cursor {
            let index = sequence.val() & self.index_mask;
            self.buffer.get(index)
        } else {
            None
        }
    }

    pub fn set(&mut self, value: T) {
        let index = self.write_cursor.next().val() & self.index_mask;
        if index == self.buffer.len() {
            self.buffer.push(value);
        } else {
            self.buffer[index] = value
        }
    }
}

/// Calculate the next power of 2, greater than or equal to `value`.
///
/// From Hacker's Delight, Chapter 3, Harry S. Warren Jr.
fn round_up_to_next_power_of_two(value: usize) -> usize {
    1_usize << mem::size_of_val(&value) - (value - 1).leading_zeros() as usize
}
