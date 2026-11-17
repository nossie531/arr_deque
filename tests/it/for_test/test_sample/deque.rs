use crate::for_test::*;
use arr_deque::prelude::*;
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

pub fn normal_tracable() -> ts::ArrDeque<ts::TraceValNt<ts::Val>, { ts::CAPACITY }> {
    normal_of()
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
