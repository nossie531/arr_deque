//! Provider of [`IntoIter`].

use crate::*;
use core::fmt::{self, Debug, Formatter};
use core::iter::FusedIterator;

/// An owning iterator over the elements of an `ArrDeque`.
///
/// This type is created by the [`into_iter`] method on [`ArrDeque`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`into_iter`]: IntoIterator::into_iter
#[derive(Clone)]
pub struct IntoIter<T, const N: usize> {
    inner: ArrDeque<T, N>,
}

impl<T, const N: usize> IntoIter<T, N> {
    pub(super) fn new(inner: ArrDeque<T, N>) -> Self {
        IntoIter { inner }
    }
}

impl<T, const N: usize> Debug for IntoIter<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.inner).finish()
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.inner.len() <= n {
            return None;
        };

        self.inner.clear_range(0..n);
        self.inner.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }

    fn count(self) -> usize {
        self.inner.len()
    }

    fn last(mut self) -> Option<Self::Item> {
        self.inner.pop_back()
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.pop_back()
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        if self.inner.len() <= n {
            return None;
        };

        let skipped_range = (self.inner.len() - n)..self.inner.len();
        self.inner.clear_range(skipped_range);
        self.inner.pop_back()
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, const N: usize> FusedIterator for IntoIter<T, N> {
    // nop.
}
