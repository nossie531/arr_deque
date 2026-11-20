use crate::for_test::*;
use arr_deque::*;

#[derive(Clone, Default)]
pub struct IterBuilder {
    arr_deque: Option<ts::SampleDeque>,
    vec_deque: Option<ts::MasterDeque>,
    head_skip: usize,
    tail_skip: usize,
}

impl IterBuilder {
    pub fn with_deque(self, value: ts::SampleDeque) -> Self {
        let vec_deque = ts::MasterDeque::from_iter(value.iter().cloned());
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

    pub fn deque(&self) -> &ts::SampleDeque {
        self.arr_deque.as_ref().unwrap()
    }

    pub fn head_skip(&self) -> usize {
        self.head_skip
    }

    pub fn tail_skip(&self) -> usize {
        self.tail_skip
    }

    pub fn each_index(self) -> impl Iterator<Item = (Self, usize)> {
        let skiped = self.head_skip + self.tail_skip;
        let len = self.arr_deque.as_ref().map_or(0, ArrDeque::len) - skiped;
        let indicies = ts::indicies(len);
        indicies.map(move |x| (self.clone(), x))
    }

    pub fn build_iter(&self) -> ts::SampleIter<'_, ts::Val> {
        if self.is_empty() {
            return ts::SampleIter::default();
        }

        let deque = self.arr_deque.as_ref().unwrap();
        let result = deque.iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_into_iter(&self) -> ts::SampleIntoIter<ts::Val> {
        if self.is_empty() {
            return ArrDeque::new().into_iter();
        }

        let deque = self.arr_deque.as_ref().unwrap().clone();
        let result = deque.into_iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_iter(&self) -> ts::MasterIter<'_, ts::Val> {
        if self.is_empty() {
            return ts::MasterIter::default();
        }

        let deque = self.vec_deque.as_ref().unwrap();
        let result = deque.iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_into_iter(&self) -> ts::MasterIntoIter<ts::Val> {
        if self.is_empty() {
            return ts::MasterDeque::new().into_iter();
        }

        let deque = self.vec_deque.as_ref().unwrap().clone();
        let result = deque.into_iter();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_iter_mut(&mut self) -> ts::SampleIterMut<'_, ts::Val> {
        if self.is_empty() {
            return ts::SampleIterMut::default();
        }

        let deque = self.arr_deque.as_mut().unwrap();
        let result = deque.iter_mut();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    pub fn build_master_iter_mut(&mut self) -> ts::MasterIterMut<'_, ts::Val> {
        if self.is_empty() {
            return ts::MasterIterMut::default();
        }

        let deque = self.vec_deque.as_mut().unwrap();
        let result = deque.iter_mut();
        tu::skip_iter(result, self.head_skip, self.tail_skip)
    }

    fn is_empty(&self) -> bool {
        self.arr_deque.is_none()
    }
}
