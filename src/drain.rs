//! Provider of [`Drain`].

use crate::*;
use core::iter::FusedIterator;
use core::ops::Range;

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

    /// Original range.
    range: Range<usize>,

    /// Current range.
    curr: Range<usize>,
}

impl<'a, T: 'a, const N: usize> Drain<'a, T, N> {
    /// Creates a new instance.
    pub(crate) fn new(target: &'a mut ArrDeque<T, N>, range: Range<usize>) -> Self {
        Self {
            target,
            range: range.clone(),
            curr: range.clone(),
        }
    }
}

impl<T, const N: usize> Drop for Drain<'_, T, N> {
    fn drop(&mut self) {
        self.target.drop_elements(&self.curr);
        self.target.clear_elements_without_drops(&self.range);
    }
}

impl<T, const N: usize> Iterator for Drain<'_, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.size_hint().1.unwrap() > 0).then_some(())?;
        let ret = unsafe { self.target.copy_val(self.curr.start) };
        self.curr.start += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.curr.len(), Some(self.curr.len()))
    }
}

impl<T, const N: usize> DoubleEndedIterator for Drain<'_, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        (self.size_hint().1.unwrap() > 0).then_some(())?;
        let ret = unsafe { self.target.copy_val(self.curr.end - 1) };
        self.curr.end -= 1;
        Some(ret)
    }
}

impl<T, const N: usize> FusedIterator for Drain<'_, T, N> {
    // nop.
}
