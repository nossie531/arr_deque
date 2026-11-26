use crate::for_test::*;
use std::ops::Range;

#[derive(Clone)]
pub struct DrainBuilder<T> {
    arr_deque: ts::SampleDequeGen<T>,
    vec_deque: ts::MasterDequeGen<T>,
    range: Range<usize>,
    head_skip: usize,
    tail_skip: usize,
}

impl<T> DrainBuilder<T> {
    pub fn new() -> Self {
        Self {
            arr_deque: ts::SampleDequeGen::new(),
            vec_deque: ts::MasterDequeGen::new(),
            range: 0..0,
            head_skip: 0,
            tail_skip: 0,
        }
    }

    pub fn with_deque(self, arr_deque: ts::SampleDequeGen<T>) -> Self
    where
        T: Clone,
    {
        let vec_deque = ts::MasterDequeGen::from_iter(arr_deque.iter().cloned());
        Self {
            vec_deque,
            arr_deque,
            ..self
        }
    }

    pub fn with_range(self, value: Range<usize>) -> Self {
        Self {
            range: value,
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
        &self.arr_deque
    }

    pub fn target_deque_mut(&mut self) -> &mut ts::SampleDequeGen<T> {
        &mut self.arr_deque
    }

    pub fn master_deque(&self) -> &ts::MasterDequeGen<T> {
        &self.vec_deque
    }

    pub fn master_deque_mut(&mut self) -> &mut ts::MasterDequeGen<T> {
        &mut self.vec_deque
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }

    pub fn build_target(&mut self) -> ts::SampleDrain<'_, T> {
        let deque = &mut self.arr_deque;
        let range = self.range.clone();
        let result = deque.drain(range.clone());
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master(&mut self) -> ts::MasterDrain<'_, T> {
        let deque = &mut self.vec_deque;
        let range = self.range.clone();
        let result = deque.drain(range.clone());
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }
}

impl DrainBuilder<ts::TraceValNt<ts::Val>> {
    pub fn new_bomb() -> Self {
        let mut bomb_deque = ts::deque::normal_traced();
        let bomb_index = bomb_deque.len() / 2;
        let bomb_value = ts::TraceValNt::new(ts::VAL).with_bomb(true);
        let range = bomb_index - 1..bomb_index + 1;
        bomb_deque.insert(bomb_index, bomb_value);
        Self::new().with_deque(bomb_deque).with_range(range)
    }
}

impl<T> Default for DrainBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
