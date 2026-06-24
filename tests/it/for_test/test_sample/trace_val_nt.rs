use crate::for_test::*;
use drop_tracer::prelude::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
use std::ops::Deref;

#[derive(Clone)]
pub(crate) struct TraceValNt(TraceVal<ts::Val>);

impl TraceValNt {
    pub fn new(value: ts::Val, tracer: &DropTracer) -> Self {
        Self(tracer.trace(value))
    }

    pub fn base_mut(&mut self) -> &mut TraceVal<ts::Val> {
        &mut self.0
    }
}

impl Distribution<TraceValNt> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TraceValNt {
        TraceValNt(TraceVal::new(rng.random()))
    }
}

impl Deref for TraceValNt {
    type Target = ts::Val;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
