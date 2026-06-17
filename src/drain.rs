//! Provider of [`Drain`].

use crate::*;
use core::ops::Range;
use std::iter::FusedIterator;

/// A draining iterator over the elements of an `ArrDeque`.
///
/// This type is created by the [`drain`] method on [`ArrDeque`].
/// See its documentation for more.
///
/// [`drain`]: ArrDeque::drain
#[derive(Debug)]
#[must_use = msg::must_use_iter!()]
pub struct Drain<'a, T: 'a, const N: usize> {
    /// Target deque.
    target: &'a mut ArrDeque<T, N>,

    /// Range to drain.
    range: Range<usize>,

    /// Drained item count from front.
    drained_front: usize,

    /// Drained item count from back.
    drained_back: usize,
}

impl<'a, T: 'a, const N: usize> Drain<'a, T, N> {
    /// Creates a new instance.
    pub(crate) fn new(target: &'a mut ArrDeque<T, N>, range: Range<usize>) -> Self {
        Self {
            target,
            range,
            drained_front: 0,
            drained_back: 0,
        }
    }
}

impl<T, const N: usize> Drop for Drain<'_, T, N> {
    fn drop(&mut self) {
        self.target.clear_range(&self.range, true);
    }
}

impl<T, const N: usize> Iterator for Drain<'_, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.size_hint().1.unwrap() > 0).then_some(())?;
        let index = self.range.start + self.drained_front;
        let ret = unsafe { self.target.copy_val(index) };
        self.drained_front += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let drained = self.drained_front + self.drained_back;
        let result = self.range.len() - drained;
        (result, Some(result))
    }
}

impl<T, const N: usize> DoubleEndedIterator for Drain<'_, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.size_hint().1.unwrap() > 0).then_some(())?;
        let index = self.range.end - self.drained_back - 1;
        let ret = unsafe { self.target.copy_val(index) };
        self.drained_back += 1;
        Some(ret)
    }
}

impl<T, const N: usize> FusedIterator for Drain<'_, T, N> {
    // nop.
}
