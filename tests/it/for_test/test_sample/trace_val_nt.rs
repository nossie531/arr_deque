use crate::for_test::*;
use drop_tracer::prelude::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
use std::ops::Rem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TraceValNt<T>(TraceVal<T>);

impl<T> TraceValNt<T> {
    pub fn new(value: T) -> Self {
        Self(TraceVal::new(value))
    }

    pub fn with_bomb(self, bomb: bool) -> Self {
        Self(TraceVal::with_bomb(self.0, bomb))
    }

    pub fn base_mut(&mut self) -> &mut TraceVal<T> {
        &mut self.0
    }

    pub fn tag(value: &Self) -> TraceTag {
        TraceVal::tag(&value.0)
    }
}

impl<'a> Rem<&'a ts::Val> for &'a TraceValNt<ts::Val> {
    type Output = ts::Val;

    fn rem(self, rhs: &ts::Val) -> Self::Output {
        *self.0 % rhs
    }
}

impl<T> Distribution<TraceValNt<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TraceValNt<T> {
        TraceValNt(TraceVal::new(rng.random()))
    }
}
