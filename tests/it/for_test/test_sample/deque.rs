use crate::for_test::*;
use arr_deque::prelude::*;

pub fn normal() -> ts::SampleDeque {
    ts::SampleBuilder::new().build()
}

pub fn type_float() -> ts::SampleDequeFloat {
    ts::SampleBuilder::new().build()
}

pub fn type_byte() -> ts::SampleDequeByte {
    ts::SampleBuilder::new().build()
}

pub fn empty() -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(0).build()
}

pub fn empty_of<T>() -> ts::ArrDeque<T, { ts::CAPACITY }> {
    ArrDeque::new()
}

pub fn custom_len(len: usize) -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(len).build()
}
