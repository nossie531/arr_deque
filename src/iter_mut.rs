//! Provider of [`IterMut`].

use crate::*;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;
use core::slice::IterMut as SliceIterMut;

/// A mutable iterator over the elements of an `ArrDeque`.
///
/// This type is created by the [`iter_mut`] method on [`ArrDeque`].
/// See its documentation for more.
///
/// [`iter_mut`]: ArrDeque::iter_mut
#[must_use = msg::must_use_iter!()]
pub struct IterMut<'a, T: 'a> {
    part1: SliceIterMut<'a, T>,
    part2: SliceIterMut<'a, T>,
}

impl<'a, T> IterMut<'a, T> {
    /// Creates a new instance.
    pub(crate) fn new(part1: SliceIterMut<'a, T>, part2: SliceIterMut<'a, T>) -> Self {
        Self { part1, part2 }
    }
}

impl<T> Debug for IterMut<'_, T>
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

impl<T> Default for IterMut<'_, T> {
    fn default() -> Self {
        Self {
            part1: Default::default(),
            part2: Default::default(),
        }
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nth_back(0)
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        util::chained_nth_back([&mut self.part1, &mut self.part2], n)
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {
    fn len(&self) -> usize {
        self.part1.len() + self.part2.len()
    }
}

impl<T> FusedIterator for IterMut<'_, T> {
    // nop.
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

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
