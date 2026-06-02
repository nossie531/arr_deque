use crate::for_test::*;
use std::ops::Range;

pub fn each_ring_starts(deque: ts::SampleDeque) -> impl Iterator<Item = (ts::SampleDeque, usize)> {
    ch::RingBufAlign::all().map(move |x| {
        let n = x.calc_ring_start(deque.len(), deque.capacity());
        (deque.clone(), n)
    })
}

pub fn each_indicies<T, const N: usize>(
    deque: ts::ArrDeque<T, N>,
) -> impl Iterator<Item = (ts::ArrDeque<T, N>, usize)>
where
    T: Clone,
{
    let (len, max) = (deque.len(), deque.capacity());
    ts::indicies(len, max).map(move |i| (deque.clone(), i))
}

pub fn each_indicies_more<T, const N: usize>(
    deque: ts::ArrDeque<T, N>,
) -> impl Iterator<Item = (ts::ArrDeque<T, N>, usize)>
where
    T: Clone,
{
    ts::indicies_more(deque.len(), deque.capacity()).map(move |i| (deque.clone(), i))
}

pub fn each_index_pairs<T, const N: usize>(
    deque: ts::ArrDeque<T, N>,
) -> impl Iterator<Item = (ts::ArrDeque<T, N>, usize, usize)>
where
    T: Clone,
{
    let (len, max) = (deque.len(), deque.capacity());
    ts::index_pairs(len, max).map(move |(i, j)| (deque.clone(), i, j))
}

pub fn each_ranges<T, const N: usize>(
    deque: ts::ArrDeque<T, N>,
) -> impl Iterator<Item = (ts::ArrDeque<T, N>, Range<usize>)>
where
    T: Clone,
{
    ts::ranges(deque.len()).map(move |r| (deque.clone(), r))
}

pub fn each_contains_keys(deque: ts::SampleDeque) -> impl Iterator<Item = (ts::SampleDeque, i32)> {
    let keys_matched = ts::indicies_inside(deque.len())
        .map(|i| deque[i])
        .collect::<Vec<_>>();

    let keys_unmatched = if deque.len() == 0 {
        const ANY_KEY: i32 = 42;
        vec![ANY_KEY]
    } else {
        let any_index = keys_matched.len() / 2;
        let key = keys_matched[any_index] + ts::UNQUANT_SIZE;
        vec![key]
    };

    let keys_all = keys_matched.into_iter().chain(keys_unmatched);
    keys_all.map(move |k| (deque.clone(), k))
}

pub fn each_search_keys(deque: ts::SampleDeque) -> impl Iterator<Item = (ts::SampleDeque, i32)> {
    let keys_matched = ts::indicies_inside(deque.len())
        .map(|i| deque[i])
        .collect::<Vec<_>>();

    let keys_unmatched = if deque.len() == 0 {
        const ANY_KEY: i32 = 42;
        vec![ANY_KEY]
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

pub fn each_predicates(
    deque: ts::SampleDeque,
) -> impl Iterator<Item = (ts::SampleDeque, Box<dyn Fn(&i32) -> bool>)> {
    ts::predicates().map(move |f| (deque.clone(), f))
}

pub fn each_predicates_mut(
    deque: ts::SampleDeque,
) -> impl Iterator<Item = (ts::SampleDeque, Box<dyn Fn(&mut i32) -> bool>)> {
    ts::predicates_mut().map(move |f| (deque.clone(), f))
}
