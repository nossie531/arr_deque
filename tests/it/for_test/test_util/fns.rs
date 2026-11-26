use crate::for_test::*;
use drop_tracer::prelude::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Mul, Rem, Sub};
use std::{array, iter};

pub fn dup<T>(x: T) -> (T, T)
where
    T: Clone,
{
    (x.clone(), x)
}

pub fn to_array<T, const N: usize>(iter: impl IntoIterator<Item = T>) -> [T; N] {
    let mut iter = iter.into_iter();
    array::from_fn(|_| iter.next().unwrap())
}

pub fn step_index(i: usize, len: usize, sep: usize) -> usize {
    let last_index = len.saturating_sub(1);
    (last_index as f32 * (i as f32 / sep as f32)) as usize
}

pub fn quantize<T>(x: T, qsize: T) -> T
where
    T: Copy + Rem<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    x - x % qsize
}

pub fn iter_dbg_text<I, T>(iter: I) -> String
where
    I: Iterator<Item = T>,
    T: Debug,
{
    let texts = iter.map(|x| format!("{x:?}"));
    let joined = texts.collect::<Vec<_>>().join(", ");
    format!("[{}]", joined)
}

pub fn skip_iter<I>(mut iter: I, head: usize, tail: usize) -> I
where
    I: DoubleEndedIterator,
{
    if head > 0 {
        iter.nth(head - 1);
    }

    if tail > 0 {
        iter.nth_back(tail - 1);
    }

    iter
}

pub fn random_vec<T>(n: usize) -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    Vec::from_iter(tu::RandIter::new().take(n))
}

pub fn hash<T: Hash>(target: &T) -> u64 {
    let hasher = &mut DefaultHasher::new();
    target.hash(hasher);
    hasher.finish()
}

pub fn buf<T>(n: usize) -> Vec<T>
where
    T: Default,
{
    Vec::from_iter(iter::repeat_with(T::default).take(n))
}

pub fn cross_iter<I>(iter: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator + Clone + 'static,
    I::Item: Clone,
{
    let iter = dup(iter);
    grid_iter(iter.0, iter.1)
}

pub fn grid_iter<IX, IY>(x: IX, y: IY) -> impl Iterator<Item = (IX::Item, IY::Item)>
where
    IX: IntoIterator,
    IY: IntoIterator + Clone + 'static,
    IX::Item: Clone,
{
    x.into_iter()
        .flat_map(move |ex| y.clone().into_iter().map(move |ey| (ex.clone(), ey)))
}

pub fn tag_map<'a, I, T: 'a>(tags: I) -> BTreeMap<TraceTag, usize>
where
    I: Iterator<Item = &'a ts::TraceValNt<T>>,
{
    BTreeMap::from_iter(
        tags.map(ts::TraceValNt::tag)
            .enumerate()
            .map(|(i, v)| (v, i)),
    )
}

pub fn eq_logs(
    xt: &DropTracer,
    yt: &DropTracer,
    xm: &BTreeMap<TraceTag, usize>,
    ym: &BTreeMap<TraceTag, usize>,
) -> bool {
    let x_logs = xt.logs();
    let y_logs = yt.logs();
    let x_iter = &mut x_logs.iter();
    let y_iter = &mut y_logs.iter();
    loop {
        match (x_iter.next(), y_iter.next()) {
            (None, None) => return true,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(x), Some(y)) => {
                let x_idx = xm.get(x.tag());
                let y_idx = ym.get(y.tag());
                if x.is_alloc() != y.is_alloc() {
                    return false;
                }

                if !matches!(x_idx.zip(y_idx), Some((x, y)) if x == y) {
                    return false;
                }
            }
        }
    }
}

pub fn trace_all<'a, I>(target: I, tracer: &DropTracer)
where
    I: Iterator<Item = &'a mut ts::TraceValNt<ts::Val>>,
{
    target.for_each(|x| tracer.trace_on(x.base_mut()));
}
