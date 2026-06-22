use crate::for_test::*;
use arr_deque::prelude::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
use subject::exts::UpgetExt;
use subject::prelude::*;

pub fn all() -> impl Iterator<Item = ts::SampleDeque> {
    builders().into_iter().map(|x| x.build())
}

pub fn all_of<T>() -> impl Iterator<Item = ArrDeque<T, { ts::CAPACITY }>>
where
    StandardUniform: Distribution<T>,
{
    builders().into_iter().map(|x| x.build())
}

pub fn all_for_contains() -> impl Iterator<Item = ts::SampleDeque> {
    all().map(for_contains)
}

pub fn all_for_binary_search() -> impl Iterator<Item = ts::SampleDeque> {
    all().map(for_binary_search)
}

pub fn none_emptys() -> impl Iterator<Item = ts::SampleDeque> {
    all().filter(|x| x.len() > 0)
}

pub fn looses() -> impl Iterator<Item = ts::SampleDeque> {
    all().filter(|x| x.len() < ts::CAPACITY)
}

pub fn fulls() -> impl Iterator<Item = ts::SampleDeque> {
    all().filter(|x| x.len() == ts::CAPACITY)
}

fn builders() -> impl Iterator<Item = ts::SampleBuilder> {
    [
        builder(0, ch::RingBufAlign::Front),
        builder(0, ch::RingBufAlign::Center),
        builder(0, ch::RingBufAlign::Back),
        builder(1, ch::RingBufAlign::Front),
        builder(1, ch::RingBufAlign::Center),
        builder(1, ch::RingBufAlign::Back),
        builder(ts::CAPACITY / 3, ch::RingBufAlign::Front),
        builder(ts::CAPACITY / 3, ch::RingBufAlign::Center),
        builder(ts::CAPACITY / 3, ch::RingBufAlign::Back),
        builder(ts::CAPACITY / 3, ch::RingBufAlign::Wrap),
        builder(ts::CAPACITY, ch::RingBufAlign::Center),
        builder(ts::CAPACITY, ch::RingBufAlign::Wrap),
    ]
    .into_iter()
}

fn builder(len: usize, align: ch::RingBufAlign) -> ts::SampleBuilder {
    let rotate = align.calc_ring_start(len, ts::CAPACITY);
    ts::DequeBuilder::new()
        .with_len(len)
        .with_ring_start(rotate)
}

fn for_contains(mut target: ts::SampleDeque) -> ts::SampleDeque {
    for i in 0..target.len() {
        target[i] = tu::quantize(target[i], ts::QUANT_SIZE);
    }

    target
}

fn for_binary_search(mut target: ts::SampleDeque) -> ts::SampleDeque {
    let values = S(target.iter().cloned().collect::<Vec<_>>()).upget(|x| x.sort());
    for i in 0..target.len() {
        target[i] = tu::quantize(values[i], ts::QUANT_SIZE);
    }

    target
}
