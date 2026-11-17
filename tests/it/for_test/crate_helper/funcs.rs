use arr_deque::prelude::*;
use std::collections::VecDeque;

pub fn master<T, const N: usize>(target: &ArrDeque<T, N>) -> VecDeque<T>
where
    T: Clone,
{
    VecDeque::from_iter(target.iter().cloned())
}
