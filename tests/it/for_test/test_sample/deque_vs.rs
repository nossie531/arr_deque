use crate::for_test::*;
use std::ops::{Range, Rem};
use std::rc::Rc;

pub fn each_ring_starts(
    deque: Rc<ts::SampleDeque>,
) -> impl Iterator<Item = (Rc<ts::SampleDeque>, usize)> {
    ch::RingBufAlign::all().map(move |x| {
        let n = x.calc_ring_start(deque.len(), deque.capacity());
        (Rc::clone(&deque), n)
    })
}

pub fn each_indices<T>(
    deque: Rc<ts::SampleDequeGen<T>>,
) -> impl Iterator<Item = (Rc<ts::SampleDequeGen<T>>, usize)>
where
    T: Clone,
{
    ts::indices(deque.len()).map(move |i| (Rc::clone(&deque), i))
}

pub fn each_indices_more<T>(
    deque: Rc<ts::SampleDequeGen<T>>,
) -> impl Iterator<Item = (Rc<ts::SampleDequeGen<T>>, usize)>
where
    T: Clone,
{
    ts::indices_more(deque.len(), deque.capacity()).map(move |i| (Rc::clone(&deque), i))
}

pub fn each_index_pairs<T>(
    deque: Rc<ts::SampleDequeGen<T>>,
) -> impl Iterator<Item = (Rc<ts::SampleDequeGen<T>>, usize, usize)>
where
    T: Clone,
{
    ts::index_pairs(deque.len()).map(move |(i, j)| (Rc::clone(&deque), i, j))
}

pub fn each_ranges<T>(
    deque: Rc<ts::SampleDequeGen<T>>,
) -> impl Iterator<Item = (Rc<ts::SampleDequeGen<T>>, Range<usize>)>
where
    T: Clone,
{
    ts::ranges(deque.len()).map(move |r| (Rc::clone(&deque), r))
}

pub fn each_predicates<T>(
    deque: Rc<ts::SampleDequeGen<T>>,
) -> impl Iterator<Item = (Rc<ts::SampleDequeGen<T>>, Box<dyn Fn(&T) -> bool>)>
where
    T: Clone,
    for<'a> &'a T: Rem<&'a ts::Val, Output = ts::Val>,
{
    ts::predicates().map(move |f| (Rc::clone(&deque), f))
}

pub fn each_predicates_mut<T>(
    deque: Rc<ts::SampleDequeGen<T>>,
) -> impl Iterator<Item = (Rc<ts::SampleDequeGen<T>>, Box<dyn Fn(&mut T) -> bool>)>
where
    T: Clone,
    for<'a> &'a T: Rem<&'a ts::Val, Output = ts::Val>,
{
    ts::predicates_mut().map(move |f| (Rc::clone(&deque), f))
}

pub fn each_contains_keys(
    deque: Rc<ts::SampleDeque>,
) -> impl Iterator<Item = (Rc<ts::SampleDeque>, ts::Val)> {
    let keys_matched = ts::indices_inside(deque.len())
        .map(|i| deque[i])
        .collect::<Vec<_>>();

    let keys_unmatched = if deque.len() == 0 {
        vec![ts::ANY_KEY]
    } else {
        let any_index = keys_matched.len() / 2;
        let key = keys_matched[any_index] + ts::UNQUANT_SIZE;
        vec![key]
    };

    let keys_all = keys_matched.into_iter().chain(keys_unmatched);
    keys_all.map(move |k| (Rc::clone(&deque), k))
}

pub fn each_search_keys(
    deque: Rc<ts::SampleDeque>,
) -> impl Iterator<Item = (Rc<ts::SampleDeque>, ts::Val)> {
    let keys_matched = ts::indices_inside(deque.len())
        .map(|i| deque[i])
        .collect::<Vec<_>>();

    let keys_unmatched = if deque.len() == 0 {
        vec![ts::ANY_KEY]
    } else {
        let min_index = 0;
        let max_index = keys_matched.len() - 1;
        let any_index = keys_matched.len() / 2;
        let min = keys_matched[min_index] - 1;
        let max = keys_matched[max_index] + ts::UNQUANT_SIZE;
        let mid = keys_matched[any_index] + ts::UNQUANT_SIZE;
        vec![min, max, mid]
    };

    let keys_all = keys_matched.into_iter().chain(keys_unmatched);
    keys_all.map(move |i| (deque.clone(), i))
}
