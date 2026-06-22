use crate::for_test::*;
use drop_tracer::prelude::*;
use rand::distr::StandardUniform;
use rand::prelude::*;
use std::ops::Deref;

#[derive(Clone)]
pub(crate) struct MyTraceVal(TraceVal<ts::Val>);

impl MyTraceVal {
    pub fn base_mut(&mut self) -> &mut TraceVal<ts::Val> {
        &mut self.0
    }
}

impl Distribution<MyTraceVal> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MyTraceVal {
        MyTraceVal(TraceVal::new(rng.random()))
    }
}

impl Deref for MyTraceVal {
    type Target = ts::Val;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
