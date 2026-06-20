use rand::prelude::*;
use rand_pcg::Pcg32;
use std::cell::RefCell;

thread_local! {
    pub(crate) static RNG: RefCell<Pcg32> = RefCell::new(Pcg32::seed_from_u64(0));
}
