use crate::for_test::*;

#[derive(Clone)]
pub struct IterBuilder {
    arr_deque: ts::SampleDeque,
    vec_deque: ts::MasterDeque,
    head_skip: usize,
    tail_skip: usize,
}

impl IterBuilder {
    pub fn new() -> Self {
        Self {
            arr_deque: ts::SampleDeque::default(),
            vec_deque: ts::MasterDeque::default(),
            head_skip: 0,
            tail_skip: 0,
        }
    }

    pub fn with_deque(self, arr_deque: ts::SampleDeque) -> Self {
        let vec_deque = ts::MasterDeque::from_iter(arr_deque.iter().cloned());
        Self {
            arr_deque,
            vec_deque,
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

    pub fn deque(&self) -> &ts::SampleDeque {
        &self.arr_deque
    }

    pub fn head_skip(&self) -> usize {
        self.head_skip
    }

    pub fn tail_skip(&self) -> usize {
        self.tail_skip
    }

    pub fn build_target_iter(&self) -> ts::SampleIter<'_, ts::Val> {
        let deque = &self.arr_deque;
        let result = deque.iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_target_into_iter(&self) -> ts::SampleIntoIter<ts::Val> {
        let deque = self.arr_deque.clone();
        let result = deque.into_iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_iter(&self) -> ts::MasterIter<'_, ts::Val> {
        let deque = &self.vec_deque;
        let result = deque.iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_into_iter(&self) -> ts::MasterIntoIter<ts::Val> {
        let deque = self.vec_deque.clone();
        let result = deque.into_iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_target_iter_mut(&mut self) -> ts::SampleIterMut<'_, ts::Val> {
        let deque = &mut self.arr_deque;
        let result = deque.iter_mut();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_iter_mut(&mut self) -> ts::MasterIterMut<'_, ts::Val> {
        let deque = &mut self.vec_deque;
        let result = deque.iter_mut();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }
}
