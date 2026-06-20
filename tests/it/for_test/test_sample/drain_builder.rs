use crate::for_test::*;
use arr_deque::*;
use std::collections::VecDeque;
use std::collections::vec_deque::Drain as Master;
use std::ops::Range;

#[derive(Clone, Default)]
pub struct DrainBuilder {
    arr_deque: Option<ts::SampleDeque>,
    vec_deque: Option<VecDeque<ts::Val>>,
    range: Option<Range<usize>>,
    head_skip: usize,
    tail_skip: usize,
}

impl DrainBuilder {
    pub fn deque(&self) -> &ts::SampleDeque {
        self.arr_deque.as_ref().unwrap()
    }

    pub fn range(&self) -> &Range<usize> {
        self.range.as_ref().unwrap()
    }

    pub fn with_deque(self, value: ts::SampleDeque) -> Self {
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

    pub fn build_target(&mut self) -> Drain<'_, ts::Val, { ts::CAPACITY }> {
        let deque = self.arr_deque.as_mut().unwrap();
        let range = self.range.as_ref().unwrap().clone();
        let result = deque.drain(range.clone());
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master(&mut self) -> Master<'_, ts::Val> {
        let deque = self.vec_deque.as_mut().unwrap();
        let range = self.range.as_ref().unwrap().clone();
        let result = deque.drain(range.clone());
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }
}
