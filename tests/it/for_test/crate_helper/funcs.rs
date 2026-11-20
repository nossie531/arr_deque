use crate::for_test::*;

pub fn master<T>(target: &ts::SampleDequeGen<T>) -> ts::MasterDequeGen<T>
where
    T: Clone,
{
    ts::MasterDequeGen::from_iter(target.iter().cloned())
}
