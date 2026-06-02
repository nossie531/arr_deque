use rand::distr::{Iter, StandardUniform};
use rand::prelude::*;
use rand_pcg::Pcg32;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::iter;
use std::ops::{Mul, Rem, Sub};

pub fn dup<T>(x: T) -> (T, T)
where
    T: Clone,
{
    (x.clone(), x)
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

pub fn random_iter<T>() -> Iter<StandardUniform, Pcg32, T>
where
    StandardUniform: Distribution<T>,
{
    Pcg32::seed_from_u64(0).random_iter()
}

pub fn random_buf<T>(n: usize) -> Vec<T>
where
    StandardUniform: Distribution<T>,
{
    Vec::from_iter(random_iter().take(n))
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
