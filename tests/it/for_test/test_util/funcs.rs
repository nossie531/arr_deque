use crate::for_test::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
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
    x.into_iter().flat_map(move |ex| y.clone().into_iter().map(move |ey| (ex.clone(), ey)))
}
