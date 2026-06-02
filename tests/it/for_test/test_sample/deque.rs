use crate::for_test::*;

pub fn normal() -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(ts::NORMAL_LEN).build()
}

pub fn type_float() -> ts::SampleDequeFloat {
    ts::SampleBuilder::new().with_len(ts::NORMAL_LEN).build()
}

pub fn type_byte() -> ts::SampleDequeByte {
    ts::SampleBuilder::new().with_len(ts::NORMAL_LEN).build()
}

pub fn empty() -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(0).build()
}

pub fn custom_len(len: usize) -> ts::SampleDeque {
    ts::SampleBuilder::new().with_len(len).build()
}
