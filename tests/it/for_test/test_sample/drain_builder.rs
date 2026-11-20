use crate::for_test::*;
use arr_deque::*;
use std::collections::vec_deque::Drain as Master;
use std::ops::Range;

#[derive(Clone)]
pub struct DrainBuilder<T> {
    arr_deque: Option<ts::SampleDequeGen<T>>,
    vec_deque: Option<ts::MasterDequeGen<T>>,
    range: Option<Range<usize>>,
    head_skip: usize,
    tail_skip: usize,
}

impl<T> DrainBuilder<T> {
    pub fn new() -> Self {
        Self {
            arr_deque: None,
            vec_deque: None,
            range: None,
            head_skip: 0,
            tail_skip: 0,
        }
    }

    pub fn with_deque(self, value: ts::SampleDequeGen<T>) -> Self
    where
        T: Clone,
    {
        let vec_deque = ts::MasterDequeGen::from_iter(value.iter().cloned());
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

    pub fn target_deque(&self) -> &ts::SampleDequeGen<T> {
        self.arr_deque.as_ref().unwrap()
    }

    pub fn target_deque_mut(&mut self) -> &mut ts::SampleDequeGen<T> {
        self.arr_deque.as_mut().unwrap()
    }

    pub fn master_deque(&self) -> &ts::MasterDequeGen<T> {
        self.vec_deque.as_ref().unwrap()
    }

    pub fn master_deque_mut(&mut self) -> &mut ts::MasterDequeGen<T> {
        self.vec_deque.as_mut().unwrap()
    }

    pub fn range(&self) -> &Range<usize> {
        self.range.as_ref().unwrap()
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

impl DrainBuilder<ts::TraceValNt<ts::Val>> {
    pub fn new_bomb() -> Self {
        let mut bomb_deque = ts::deque::normal_traced();
        let bomb_index = bomb_deque.len() / 2;
        let bomb_value = ts::TraceValNt::new(ts::VAL).with_bomb(true);
        let range = bomb_index-1..bomb_index+1;
        bomb_deque.insert(bomb_index, bomb_value);
        Self::new().with_deque(bomb_deque).with_range(range)
    }
}

impl<T> Default for DrainBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
