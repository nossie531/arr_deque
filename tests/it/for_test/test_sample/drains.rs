use crate::for_test::*;
use rand::distr::{Distribution, StandardUniform};
use std::iter;

pub fn all() -> impl Iterator<Item = ts::DrainBuilder<ts::Val>> {
    all_of()
}

pub fn all_of<T>() -> impl Iterator<Item = ts::DrainBuilder<T>>
where
    T: Clone,
    StandardUniform: Distribution<T>,
{
    let deques = ts::deques::all_of::<T>();
    deques
        .map(|x| ts::DrainBuilder::default().with_deque(x))
        .flat_map(each_ranges)
        .flat_map(each_skips)
}

pub fn all_with_bomb() -> impl Iterator<Item = ts::DrainBuilder<ts::TraceValNt<ts::Val>>> {
    all_of().chain(iter::once_with(ts::DrainBuilder::new_bomb))
}

fn each_ranges<T>(builder: ts::DrainBuilder<T>) -> impl Iterator<Item = ts::DrainBuilder<T>>
where
    T: Clone,
{
    let ranges = ts::ranges_inside(builder.target_deque().len());
    ranges.map(move |x| builder.clone().with_range(x))
}

fn each_skips<T>(builder: ts::DrainBuilder<T>) -> impl Iterator<Item = ts::DrainBuilder<T>>
where
    T: Clone,
{
    let range_len = builder.range().len();
    let ranges_in_range = || ts::ranges_inside(builder.range().len());
    let head_skips = ranges_in_range().map(|x| x.start);
    let tail_skips = ranges_in_range().map(move |x| range_len - x.end);
    let skips = head_skips.zip(tail_skips);
    skips.map(move |x| builder.clone().with_head_skip(x.0).with_tail_skip(x.1))
}
