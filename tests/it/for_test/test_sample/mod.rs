pub(crate) mod deque;
pub(crate) mod deque_pairs;
pub(crate) mod deque_vs;
pub(crate) mod deques;
pub(crate) mod drains;
pub(crate) mod iter_vs;
pub(crate) mod iters;

pub(crate) use deque_builder::*;
pub(crate) use drain_builder::*;
pub(crate) use fns::*;
pub(crate) use iter_builder::*;
pub(crate) use trace_val_nt::*;

pub const ANY_KEY: i32 = 42;
pub const VAL: Val = 42;
pub const QUANT_SIZE: Val = 10;
pub const UNQUANT_SIZE: Val = QUANT_SIZE - 1;
pub const CAPACITY: usize = 30;
pub const NORMAL_RATIO: f32 = 0.3;
pub const NORMAL_LEN: usize = (CAPACITY as f32 * NORMAL_RATIO) as usize;

pub type Val = i32;
pub type Float = f32;
pub type SampleBuilder = DequeBuilder<CAPACITY>;
pub type SampleDeque = SampleDequeGen<Val>;
pub type SampleDequeFloat = SampleDequeGen<Float>;
pub type SampleDequeByte = SampleDequeGen<u8>;
pub type SampleDequeTraced = SampleDequeGen<TraceValNt<Val>>;
pub type SampleDequeGen<T> = arr_deque::ArrDeque<T, CAPACITY>;
pub type SampleIntoIter<T> = arr_deque::IntoIter<T, CAPACITY>;
pub type SampleIter<'a, T> = arr_deque::Iter<'a, T>;
pub type SampleIterMut<'a, T> = arr_deque::IterMut<'a, T>;
pub type SampleDrain<'a, T> = arr_deque::Drain<'a, T, CAPACITY>;
pub type MasterDeque = std::collections::vec_deque::VecDeque<Val>;
pub type MasterDequeGen<T> = std::collections::vec_deque::VecDeque<T>;
pub type MasterIntoIter<T> = std::collections::vec_deque::IntoIter<T>;
pub type MasterIter<'a, T> = std::collections::vec_deque::Iter<'a, T>;
pub type MasterIterMut<'a, T> = std::collections::vec_deque::IterMut<'a, T>;
pub type MasterDrain<'a, T> = std::collections::vec_deque::Drain<'a, T>;

mod deque_builder;
mod drain_builder;
mod fns;
mod iter_builder;
mod trace_val_nt;
