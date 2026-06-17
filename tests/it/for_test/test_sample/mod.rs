pub(crate) mod deque;
pub(crate) mod deque_pairs;
pub(crate) mod deque_vs;
pub(crate) mod deques;
pub(crate) mod drains;
pub(crate) mod iters;
pub(crate) use deque_builder::*;
pub(crate) use drain_builder::*;
pub(crate) use funcs::*;
pub(crate) use iter_builder::*;

pub const VAL: i32 = 42;
pub const QUANT_SIZE: i32 = 10;
pub const UNQUANT_SIZE: i32 = QUANT_SIZE - 1;
pub const CAPACITY: usize = 30;
pub const NORMAL_RATIO: f32 = 0.3;
pub const NORMAL_LEN: usize = (CAPACITY as f32 * NORMAL_RATIO) as usize;

pub type SampleDeque = ArrDeque<i32, CAPACITY>;
pub type SampleDequeFloat = ArrDeque<f32, CAPACITY>;
pub type SampleDequeByte = ArrDeque<u8, CAPACITY>;
pub type SampleBuilder = DequeBuilder<CAPACITY>;

mod deque_builder;
mod drain_builder;
mod funcs;
mod iter_builder;
use arr_deque::prelude::*;
