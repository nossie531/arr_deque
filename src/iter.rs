//! Provider of [`Iter`].

use crate::*;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::slice::Iter as SliceIter;

/// An iterator over the elements of an `ArrDeque`.
///
/// This type is created by the [`iter`] method on [`ArrDeque`].
/// See its documentation for more.
///
/// [`iter`]: ArrDeque::iter
#[must_use = msg::must_use_iter!()]
pub struct Iter<'a, T: 'a> {
    part1: SliceIter<'a, T>,
    part2: SliceIter<'a, T>,
}

impl<'a, T> Iter<'a, T> {
    /// Creates a new instance.
    pub(crate) fn new(part1: SliceIter<'a, T>, part2: SliceIter<'a, T>) -> Self {
        Self { part1, part2 }
    }
}

impl<T> Clone for Iter<'_, T> {
    fn clone(&self) -> Self {
        Self {
            part1: self.part1.clone(),
            part2: self.part2.clone(),
        }
    }
}

impl<T> Debug for Iter<'_, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter")
            .field(&self.part1)
            .field(&self.part2)
            .finish()
    }
}

impl<T> Default for Iter<'_, T> {
    fn default() -> Self {
        Self {
            part1: Default::default(),
            part2: Default::default(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.nth(0)
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        util::chained_nth([&mut self.part1, &mut self.part2], n)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nth_back(0)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        util::chained_nth_back([&mut self.part1, &mut self.part2], n)
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {
    fn len(&self) -> usize {
        self.part1.len() + self.part2.len()
    }
}

impl<T> FusedIterator for Iter<'_, T> {
    // nop.
}
