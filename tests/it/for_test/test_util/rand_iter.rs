use crate::for_test::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
use std::marker::PhantomData;

pub(crate) struct RandIter<T>(PhantomData<T>);

impl<T> RandIter<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<T> Iterator for RandIter<T>
where
    StandardUniform: Distribution<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        tu::RNG.with_borrow_mut(|x| Some(x.random()))
    }
}
