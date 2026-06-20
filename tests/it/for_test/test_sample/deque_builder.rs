use crate::for_test::*;
use arr_deque::prelude::*;
use rand::distr::{Distribution, StandardUniform};

pub struct DequeBuilder<const N: usize> {
    len: usize,
    ring_start: usize,
}

impl<const N: usize> DequeBuilder<N> {
    pub fn new() -> Self {
        Self {
            len: (N as f32 * ts::NORMAL_RATIO) as usize,
            ring_start: 0,
        }
    }

    pub fn with_len(mut self, value: usize) -> Self {
        assert!(value <= N);
        self.len = value;
        self
    }

    pub fn with_ring_start(mut self, value: usize) -> Self {
        assert!(value < N);
        self.ring_start = value;
        self
    }

    pub fn build<T>(&self) -> ArrDeque<T, N>
    where
        StandardUniform: Distribution<T>,
    {
        let mut ret = ArrDeque::new();
        ret.adjust_ring_start(self.ring_start);
        ret.extend(tu::RandIter::new().take(self.len));
        ret
    }
}
