//! Deque implemented by array.
//!
//! _The author of this crate is not good at English._  
//! _Forgive me if the document is hard to read._
//!
//! # Core item
//!
//! - [`ArrDeque`] - Deque by array

#![no_std]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod prelude;
pub use arr_deque::*;
pub use drain::*;
pub use into_iter::*;
pub use iter::*;
pub use iter_mut::*;

mod arr_deque;
mod drain;
mod into_iter;
mod iter;
mod iter_mut;
mod msg;
mod util;
