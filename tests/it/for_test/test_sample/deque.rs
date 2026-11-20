use crate::for_test::*;
use rand::distr::{Distribution, StandardUniform};

pub fn empty() -> ts::SampleDeque {
    empty_of()
}

pub fn empty_of<T>() -> ts::SampleDequeGen<T> {
    ts::SampleDequeGen::new()
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

pub fn normal_traced() -> ts::SampleDequeTraced {
    normal_of()
}

pub fn normal_of<T>() -> ts::SampleDequeGen<T>
where
    StandardUniform: Distribution<T>,
{
    ts::SampleBuilder::new().build()
}

pub fn with_bomb() -> ts::SampleDequeTraced {
    let mut ret = ts::deque::normal_traced();
    let bomb_index = ret.len() / 2;
    let bomb_value = ts::TraceValNt::new(ts::VAL).with_bomb(true);
    ret.insert(bomb_index, bomb_value);
    ret
}

pub fn custom_len(len: usize) -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(len).build()
}
