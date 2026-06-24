use crate::for_test::*;
use arr_deque::prelude::*;
use drop_tracer::DropTracer;
use rand::distr::{Distribution, StandardUniform};

pub fn empty() -> ts::SampleDeque {
    empty_of()
}

pub fn empty_of<T>() -> ts::ArrDeque<T, { ts::CAPACITY }> {
    ArrDeque::new()
}

pub fn normal() -> ts::SampleDeque {
    normal_of()
}

pub fn normal_float() -> ts::SampleDequeFloat {
    normal_of()
}

pub fn normal_byte() -> ts::SampleDequeByte {
    normal_of()
}

pub fn normal_traced(tracer: &DropTracer) -> ts::ArrDeque<ts::TraceValNt, { ts::CAPACITY }> {
    let mut ret = normal_of::<ts::TraceValNt>();
    for value in ret.iter_mut() {
        tracer.trace_on(value.base_mut());
    }

    ret
}

pub fn normal_of<T>() -> ts::ArrDeque<T, { ts::CAPACITY }>
where
    StandardUniform: Distribution<T>,
{
    ts::SampleBuilder::new().build()
}

pub fn custom_len(len: usize) -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(len).build()
}
