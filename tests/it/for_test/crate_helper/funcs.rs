use crate::for_test::crate_helper::*;
use arr_deque::prelude::*;
use std::collections::VecDeque;

pub fn master<T, const N: usize>(target: &ArrDeque<T, N>) -> VecDeque<T>
where
    T: Clone,
{
    VecDeque::from_iter(target.iter().cloned())
}

pub fn adjust_ring_start<T, const N: usize>(
    mut target: ArrDeque<T, N>,
    align: RingBufAlign,
) -> ArrDeque<T, N> {
    let len = target.len();
    let capacity = target.capacity();
    let ring_start = align.calc_ring_start(len, capacity);
    target.adjust_ring_start(ring_start);
    target
}
