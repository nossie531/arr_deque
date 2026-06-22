use crate::for_test::*;
use arr_deque::*;
use std::collections::VecDeque;
use std::collections::vec_deque::Drain as Master;
use std::ops::Range;

#[derive(Clone)]
pub struct DrainBuilder<T> {
    arr_deque: Option<ArrDeque<T, { ts::CAPACITY }>>,
    vec_deque: Option<VecDeque<T>>,
    range: Option<Range<usize>>,
    head_skip: usize,
    tail_skip: usize,
}

impl<T> DrainBuilder<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            arr_deque: None,
            vec_deque: None,
            range: None,
            head_skip: 0,
            tail_skip: 0,
        }
    }

    pub fn deque(&self) -> &ArrDeque<T, { ts::CAPACITY }> {
        self.arr_deque.as_ref().unwrap()
    }

    pub fn deque_mut(&mut self) -> &mut ArrDeque<T, { ts::CAPACITY }> {
        self.arr_deque.as_mut().unwrap()
    }

    pub fn range(&self) -> &Range<usize> {
        self.range.as_ref().unwrap()
    }

    pub fn with_deque(self, value: ArrDeque<T, { ts::CAPACITY }>) -> Self {
        let vec_deque = VecDeque::from_iter(value.iter().cloned());
        Self {
            arr_deque: Some(value),
            vec_deque: Some(vec_deque),
            ..self
        }
    }

    pub fn with_range(self, value: Range<usize>) -> Self {
        Self {
            range: Some(value),
            ..self
        }
    }

    pub fn with_head_skip(self, value: usize) -> Self {
        Self {
            head_skip: value,
            ..self
        }
    }

    pub fn with_tail_skip(self, value: usize) -> Self {
        Self {
            tail_skip: value,
            ..self
        }
    }

    pub fn build_target(&mut self) -> Drain<'_, T, { ts::CAPACITY }> {
        let deque = self.arr_deque.as_mut().unwrap();
        let range = self.range.as_ref().unwrap().clone();
        let result = deque.drain(range.clone());
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master(&mut self) -> Master<'_, T> {
        let deque = self.vec_deque.as_mut().unwrap();
        let range = self.range.as_ref().unwrap().clone();
        let result = deque.drain(range.clone());
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }
}

impl<T> Default for DrainBuilder<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
