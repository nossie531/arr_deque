use crate::for_test::*;
use arr_deque::*;
use std::collections::VecDeque;
use std::collections::vec_deque::IntoIter as MasterIntoIter;
use std::collections::vec_deque::Iter as MasterIter;
use std::collections::vec_deque::IterMut as MasterIterMut;

#[derive(Clone, Default)]
pub struct IterBuilder {
    arr_deque: Option<ts::SampleDeque>,
    vec_deque: Option<VecDeque<i32>>,
    head_skip: usize,
    tail_skip: usize,
}

impl IterBuilder {
    pub fn deque(&self) -> &ts::SampleDeque {
        self.arr_deque.as_ref().unwrap()
    }

    pub fn head_skip(&self) -> usize {
        self.head_skip
    }

    pub fn tail_skip(&self) -> usize {
        self.tail_skip
    }

    pub fn with_deque(self, value: ts::SampleDeque) -> Self {
        let vec_deque = VecDeque::from_iter(value.iter().cloned());
        Self {
            arr_deque: Some(value),
            vec_deque: Some(vec_deque),
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

    pub fn each_index(self) -> impl Iterator<Item = (Self, usize)> {
        let skiped = self.head_skip + self.tail_skip;
        let len = self.deque().len() - skiped;
        let max = self.deque().capacity() - skiped;
        let ns = ts::indicies(len, max);
        ns.map(move |x| (self.clone(), x))
    }

    pub fn build_iter(&self) -> Iter<'_, ts::Val> {
        let deque = self.arr_deque.as_ref().unwrap();
        let result = deque.iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_into_iter(&self) -> IntoIter<ts::Val, { ts::CAPACITY }> {
        let deque = self.arr_deque.as_ref().unwrap().clone();
        let result = deque.into_iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_iter(&self) -> MasterIter<'_, ts::Val> {
        let deque = self.vec_deque.as_ref().unwrap();
        let result = deque.iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_into_iter(&self) -> MasterIntoIter<ts::Val> {
        let deque = self.vec_deque.as_ref().unwrap().clone();
        let result = deque.into_iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_iter_mut(&mut self) -> IterMut<'_, ts::Val> {
        let deque = self.arr_deque.as_mut().unwrap();
        let result = deque.iter_mut();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_iter_mut(&mut self) -> MasterIterMut<'_, ts::Val> {
        let deque = self.vec_deque.as_mut().unwrap();
        let result = deque.iter_mut();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }
}
