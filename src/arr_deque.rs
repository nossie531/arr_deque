//! Provider of [`ArrDeque`].

use crate::util::*;
use crate::*;
use core::array;
use core::cmp::Ordering;
use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::mem;
use core::mem::MaybeUninit;
use core::ops::{Index, IndexMut, Not, Range, RangeBounds};
use core::ptr;
use std::io::{self, BufRead, Read, Write};
use subject::exts::{MapExt, UpgetExt};
use subject::prelude::*;

#[cfg(feature = "alloc")]
use std::vec::Vec;

#[cfg(doc)]
use std::collections::VecDeque;

/// A double-ended queue implemented with an array.
///
/// This item is similar to [`VecDeque`]. However, this type uses
/// arrays instead of vectors for its internal ring buffer.
///
/// # Examples
///
/// ```
/// use arr_deque::prelude::*;
///
/// let deque = &mut ArrDeque::<_, 30>::new();
/// deque.push_back(1);
/// deque.push_back(2);
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_front(), Some(2));
/// assert_eq!(deque.pop_front(), None);
/// ```
///
/// # Differences from [`VecDeque`]
///
/// Some methods behave slightly differ from [`VecDeque`] (Rust 1.96.0).
///
/// - **Allocating**
///
///   The following methods are not implemented.
///
///   - [`VecDeque::reserve`]
///   - [`VecDeque::reserve_exact`]
///   - [`VecDeque::shrink_to`]
///   - [`VecDeque::shrink_to_fit`]
///   - [`VecDeque::try_reserve`]
///   - [`VecDeque::try_reserve_exact`]
///   - [`VecDeque::with_capacity`]
///
/// - **Panic by buffer limit**
///
///   The following methods panic when the buffer limit is reached.
///
///   - [`append`](Self::append)
///   - [`insert`](Self::insert)
///   - [`insert_mut`](Self::insert_mut)
///   - [`push_back`](Self::push_back)
///   - [`push_back_mut`](Self::push_back_mut)
///   - [`push_front`](Self::push_front)
///   - [`push_front_mut`](Self::push_front_mut)
///   - [`resize`](Self::resize)
///   - [`resize_with`](Self::resize_with)
///   - [`Extend`] methods
///   - [`From`] methods
///   - [`FromIterator`] methods
///   - [`Write`] methods
///
/// - **Buffer split position**
///
///   The following methods are affected by the buffer split position.
///   And the exact split position is not guaranteed in specification.
///
///   - [`as_slices`](Self::as_slices)
///   - [`as_mut_slices`](Self::as_mut_slices)
///   - [`BufRead`] methods
///   - [`Read`] methods
///
/// - **Original**
///
///   The following methods are not implemented on [`VecDeque`].
///
///   - [`adjust_ring_start`](Self::adjust_ring_start)
pub struct ArrDeque<T, const N: usize> {
    /// Length.
    len: usize,

    /// Start index of the ring buffer.
    start: BufIdx,

    /// Array for the ring buffer.
    buf: [MaybeUninit<T>; N],
}

/// Methods like [`VecDeque`].
///
/// [`VecDeque`]: std::collections::VecDeque
impl<T, const N: usize> ArrDeque<T, N> {
    /// Creates an empty deque.
    ///
    /// See [`VecDeque::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<u32, 30> = ArrDeque::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            start: 0,
            len: 0,
            buf: [const { MaybeUninit::uninit() }; N],
        }
    }

    /// Returns `true` if the deque is empty.
    ///
    /// See [`VecDeque::is_empty`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque = ArrDeque::<_, 30>::new();
    /// assert!(deque.is_empty());
    /// deque.push_front(1);
    /// assert!(!deque.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if this deque contains given value.
    ///
    /// See [`VecDeque::contains`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<_, 30> = ArrDeque::new();
    ///
    /// deque.push_back(0);
    /// deque.push_back(1);
    ///
    /// assert_eq!(deque.contains(&1), true);
    /// assert_eq!(deque.contains(&10), false);
    /// ```
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>,
    {
        self.iter().any(|item| item.eq(x))
    }

    /// Returns the number of elements this deque can hold.
    ///
    /// See [`VecDeque::capacity`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let buf: ArrDeque<i32, 10> = ArrDeque::new();
    /// assert!(buf.capacity() == 10);
    /// ```
    pub fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of elements.
    ///
    /// See [`VecDeque::len`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque = ArrDeque::<_, 30>::new();
    /// assert_eq!(deque.len(), 0);
    /// deque.push_back(1);
    /// assert_eq!(deque.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns a reference to the front element.
    ///
    /// See [`VecDeque::front`].
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::new();
    /// assert_eq!(d.front(), None);
    ///
    /// d.push_back(1);
    /// d.push_back(2);
    /// assert_eq!(d.front(), Some(&1));
    /// ```
    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    /// Returns a mutable reference to the font element.
    ///
    /// See [`VecDeque::front_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::new();
    /// assert_eq!(d.front_mut(), None);
    ///
    /// d.push_back(1);
    /// d.push_back(2);
    /// match d.front_mut() {
    ///     Some(x) => *x = 9,
    ///     None => (),
    /// }
    /// assert_eq!(d.front(), Some(&9));
    /// ```
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Returns a reference to the back element.
    ///
    /// See [`VecDeque::back`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::new();
    /// assert_eq!(d.back(), None);
    ///
    /// d.push_back(1);
    /// d.push_back(2);
    /// assert_eq!(d.back(), Some(&2));
    /// ```
    pub fn back(&self) -> Option<&T> {
        self.get(self.len.checked_sub(1)?)
    }

    /// Returns a mutable reference to the back element.
    ///
    /// See [`VecDeque::back_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::new();
    /// assert_eq!(d.back(), None);
    ///
    /// d.push_back(1);
    /// d.push_back(2);
    /// match d.back_mut() {
    ///     Some(x) => *x = 9,
    ///     None => (),
    /// }
    /// assert_eq!(d.back(), Some(&9));
    /// ```
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len.checked_sub(1)?)
    }

    /// Returns a reference to the element at the given index.
    ///
    /// See [`VecDeque::get`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(3);
    /// buf.push_back(4);
    /// buf.push_back(5);
    /// buf.push_back(6);
    /// assert_eq!(buf.get(1), Some(&4));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }

        let elm_ptr = self.buf[self.to_buf_idx(index)].as_ptr();
        let elm_ref = unsafe { &*elm_ptr };
        Some(elm_ref)
    }

    /// Returns a mutable reference to the element at the given index.
    ///
    /// See [`VecDeque::get_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(3);
    /// buf.push_back(4);
    /// buf.push_back(5);
    /// buf.push_back(6);
    /// assert_eq!(buf[1], 4);
    /// if let Some(elem) = buf.get_mut(1) {
    ///     *elem = 7;
    /// }
    /// assert_eq!(buf[1], 7);
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }

        let elm_ptr = self.buf[self.to_buf_idx(index)].as_mut_ptr();
        let elm_mut = unsafe { &mut *elm_ptr };
        Some(elm_mut)
    }

    /// Two slices of the ring buffer array.
    ///
    /// See [`VecDeque::as_slices`].
    ///
    /// # Notes
    ///
    /// Split point of the slice may differ from the modeled item.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque = ArrDeque::<_, 30>::new();
    ///
    /// deque.push_back(0);
    /// deque.push_back(1);
    /// deque.push_back(2);
    ///
    /// let expected = [0, 1, 2];
    /// let (front, back) = deque.as_slices();
    /// assert_eq!(&expected[..front.len()], front);
    /// assert_eq!(&expected[front.len()..], back);
    ///
    /// deque.push_front(10);
    /// deque.push_front(9);
    ///
    /// let expected = [9, 10, 0, 1, 2];
    /// let (front, back) = deque.as_slices();
    /// assert_eq!(&expected[..front.len()], front);
    /// assert_eq!(&expected[front.len()..], back);
    /// ```
    pub fn as_slices(&self) -> (&[T], &[T]) {
        self.to_slices(&self.all()).into()
    }

    /// Two mutable slices of the ring buffer array.
    ///
    /// See [`VecDeque::as_mut_slices`].
    ///
    /// # Notes
    ///
    /// Split point of the slice may differ from the modeled item.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque = ArrDeque::<_, 30>::new();
    ///
    /// deque.push_back(0);
    /// deque.push_back(1);
    ///
    /// deque.push_front(10);
    /// deque.push_front(9);
    ///
    /// // Since the split point is not guaranteed, we may need to update
    /// // either slice.
    /// let mut update_nth = |index: usize, val: u32| {
    ///     let (front, back) = deque.as_mut_slices();
    ///     if index > front.len() - 1 {
    ///         back[index - front.len()] = val;
    ///     } else {
    ///         front[index] = val;
    ///     }
    /// };
    ///
    /// update_nth(0, 42);
    /// update_nth(2, 24);
    ///
    /// let v: Vec<_> = deque.into();
    /// assert_eq!(v, [42, 10, 24, 1]);
    /// ```
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        self.to_slices_mut(&self.all()).into()
    }

    /// Returns an iterator.
    ///
    /// See [`VecDeque::iter`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(5);
    /// buf.push_back(3);
    /// buf.push_back(4);
    /// let b: &[_] = &[&5, &3, &4];
    /// let c: Vec<&i32> = buf.iter().collect();
    /// assert_eq!(&c[..], b);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        let (s1, s2) = self.as_slices();
        Iter::new(s1.iter(), s2.iter())
    }

    /// Returns an mutable iterator.
    ///
    /// See [`VecDeque::iter_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(5);
    /// buf.push_back(3);
    /// buf.push_back(4);
    /// for num in buf.iter_mut() {
    ///     *num = *num - 2;
    /// }
    /// let b: &[_] = &[&mut 3, &mut 1, &mut 2];
    /// assert_eq!(&buf.iter_mut().collect::<Vec<&mut i32>>()[..], b);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let (s1, s2) = self.as_mut_slices();
        IterMut::new(s1.iter_mut(), s2.iter_mut())
    }

    /// Returns an iterator that covers the given range.
    ///
    /// See [`VecDeque::range`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<_, 30> = [1, 2, 3].into();
    /// let range = deque.range(2..).copied().collect::<ArrDeque<_, 30>>();
    /// assert_eq!(range, [3]);
    ///
    /// // A full range covers all contents
    /// let all = deque.range(..);
    /// assert_eq!(all.len(), 3);
    /// ```
    pub fn range<R>(&self, range: R) -> Iter<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let (s1, s2) = self.to_slices(&util::slice_range(range, ..self.len)).into();
        Iter::new(s1.iter(), s2.iter())
    }

    /// Returns a mutable iterator that covers the given range.
    ///
    /// See [`VecDeque::range_mut`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<_, 30> = [1, 2, 3].into();
    /// for v in deque.range_mut(2..) {
    ///   *v *= 2;
    /// }
    /// assert_eq!(deque, [1, 2, 6]);
    ///
    /// // A full range covers all contents
    /// for v in deque.range_mut(..) {
    ///   *v *= 2;
    /// }
    /// assert_eq!(deque, [2, 4, 12]);
    /// ```
    pub fn range_mut<R>(&mut self, range: R) -> IterMut<'_, T>
    where
        R: RangeBounds<usize>,
    {
        let (s1, s2) = self
            .to_slices_mut(&util::slice_range(range, ..self.len))
            .into();
        IterMut::new(s1.iter_mut(), s2.iter_mut())
    }

    /// Returns the partition index according to the given predicate.
    ///
    /// See [`VecDeque::partition_point`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<_, 30> = [1, 2, 3, 3, 5, 6, 7].into();
    /// let i = deque.partition_point(|&x| x < 5);
    ///
    /// assert_eq!(i, 4);
    /// assert!(deque.iter().take(i).all(|&x| x < 5));
    /// assert!(deque.iter().skip(i).all(|&x| !(x < 5)));
    /// ```
    ///
    /// If you want to insert an item to a sorted deque, while maintaining sort order:
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<_, 30> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
    /// let num = 42;
    /// let idx = deque.partition_point(|&x| x < num);
    /// deque.insert(idx, num);
    /// assert_eq!(deque, &[0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    /// ```
    pub fn partition_point<P>(&self, mut pred: P) -> usize
    where
        P: FnMut(&T) -> bool,
    {
        let (s1, s2) = self.as_slices();
        if s2.first().is_some_and(&mut pred) {
            s2.partition_point(pred) + s1.len()
        } else {
            s1.partition_point(pred)
        }
    }

    /// Binary searches this for given element.
    ///
    /// See [`VecDeque::binary_search`].
    ///
    /// # Examples
    ///
    /// Looks up a series of four elements. The first is found, with a uniquely
    /// determined position; the second and third are not found; the fourth
    /// could match any position in `[1, 4]`.
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<_, 30> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
    ///
    /// assert_eq!(deque.binary_search(&13),  Ok(9));
    /// assert_eq!(deque.binary_search(&4),   Err(7));
    /// assert_eq!(deque.binary_search(&100), Err(13));
    /// let r = deque.binary_search(&1);
    /// assert!(matches!(r, Ok(1..=4)));
    /// ```
    ///
    /// If you want to insert an item to a sorted deque, while maintaining
    /// sort order, consider using [partition_point](Self::partition_point):
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<_, 30> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
    /// let num = 42;
    /// let idx = deque.partition_point(|&x| x <= num);
    /// // If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
    /// // `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` may allow `insert`
    /// // to shift less elements.
    /// deque.insert(idx, num);
    /// assert_eq!(deque, &[0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    /// ```
    pub fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.binary_search_by(|e| e.cmp(x))
    }

    /// Binary searches this with a comparator function.
    ///
    /// See [`VecDeque::binary_search_by`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<_, 30> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
    ///
    /// assert_eq!(deque.binary_search_by(|x| x.cmp(&13)),  Ok(9));
    /// assert_eq!(deque.binary_search_by(|x| x.cmp(&4)),   Err(7));
    /// assert_eq!(deque.binary_search_by(|x| x.cmp(&100)), Err(13));
    /// let r = deque.binary_search_by(|x| x.cmp(&1));
    /// assert!(matches!(r, Ok(1..=4)));
    /// ```
    pub fn binary_search_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> Ordering,
    {
        let mut range = self.all();
        while !range.is_empty() {
            let mid = range.start + (range.end - range.start) / 2;
            let val = self.get(mid).unwrap();
            match f(val) {
                Ordering::Equal => return Ok(mid),
                Ordering::Less => range = (mid + 1)..range.end,
                Ordering::Greater => range = range.start..mid,
            }
        }

        Err(range.start)
    }

    /// Binary searches this with a key extraction function.
    ///
    /// See [`VecDeque::binary_search_by_key`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<_, 30> = [(0, 0), (2, 1), (4, 1), (5, 1),
    ///          (3, 1), (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
    ///          (1, 21), (2, 34), (4, 55)].into();
    ///
    /// assert_eq!(deque.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
    /// assert_eq!(deque.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
    /// assert_eq!(deque.binary_search_by_key(&100, |&(a, b)| b), Err(13));
    /// let r = deque.binary_search_by_key(&1, |&(a, b)| b);
    /// assert!(matches!(r, Ok(1..=4)));
    /// ```
    pub fn binary_search_by_key<'a, B, F>(&'a self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a T) -> B,
        B: Ord,
    {
        self.binary_search_by(|k| f(k).cmp(b))
    }

    /// Clears the deque, removing all values.
    ///
    /// See [`VecDeque::clear`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque = ArrDeque::<_, 30>::new();
    /// deque.push_back(1);
    /// deque.clear();
    /// assert!(deque.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.truncate(0);
        self.start = 0;
    }

    /// Removes an element from tail of this deque.
    ///
    /// See [`VecDeque::pop_front`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::new();
    /// d.push_back(1);
    /// d.push_back(2);
    ///
    /// assert_eq!(d.pop_front(), Some(1));
    /// assert_eq!(d.pop_front(), Some(2));
    /// assert_eq!(d.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.remove(0)
    }

    /// Removes an element from tail of this deque.
    ///
    /// See [`VecDeque::pop_back`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// assert_eq!(buf.pop_back(), None);
    /// buf.push_back(1);
    /// buf.push_back(3);
    /// assert_eq!(buf.pop_back(), Some(3));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.remove(self.len().checked_sub(1)?)
    }

    /// Removes an element from head of this deque if matched given predicate.
    ///
    /// See [`VecDeque::pop_front_if`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<i32, 30> = vec![0, 1, 2, 3, 4].into();
    /// let pred = |x: &mut i32| *x % 2 == 0;
    ///
    /// assert_eq!(deque.pop_front_if(pred), Some(0));
    /// assert_eq!(deque, [1, 2, 3, 4]);
    /// assert_eq!(deque.pop_front_if(pred), None);
    /// ```
    pub fn pop_front_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        predicate(self.front_mut()?)
            .then(|| self.pop_front())
            .flatten()
    }

    /// Removes an element from tail of this deque if matched given predicate.
    ///
    /// See [`VecDeque::pop_back_if`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<i32, 30> = vec![0, 1, 2, 3, 4].into();
    /// let pred = |x: &mut i32| *x % 2 == 0;
    ///
    /// assert_eq!(deque.pop_back_if(pred), Some(4));
    /// assert_eq!(deque, [0, 1, 2, 3]);
    /// assert_eq!(deque.pop_back_if(pred), None);
    /// ```
    pub fn pop_back_if(&mut self, predicate: impl FnOnce(&mut T) -> bool) -> Option<T> {
        predicate(self.back_mut()?)
            .then(|| self.pop_back())
            .flatten()
    }

    /// Prepends an element to this deque.
    ///
    /// See [`VecDeque::push_front`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::new();
    /// d.push_front(1);
    /// d.push_front(2);
    /// assert_eq!(d.front(), Some(&2));
    /// ```
    pub fn push_front(&mut self, value: T) {
        assert!(self.len() < N, msg::cap_over_addition!(), N);
        let _ = self.push_front_mut(value);
    }

    /// Prepends an element to this deque, returning a reference to it.
    ///
    /// See [`VecDeque::push_front_mut`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::from([1, 2, 3]);
    /// let x = d.push_front_mut(8);
    /// *x -= 1;
    /// assert_eq!(d.front(), Some(&7));
    /// ```
    #[must_use = msg::must_use_reference!("ArrDeque::push_front")]
    pub fn push_front_mut(&mut self, value: T) -> &mut T {
        assert!(self.len() < N, msg::cap_over_addition!(), N);
        self.insert_mut(0, value)
    }

    /// Appends an element to this deque.
    ///
    /// See [`VecDeque::push_back`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(1);
    /// buf.push_back(3);
    /// assert_eq!(3, *buf.back().unwrap());
    /// ```
    pub fn push_back(&mut self, value: T) {
        assert!(self.len() < N, msg::cap_over_addition!(), N);
        let _ = self.push_back_mut(value);
    }

    /// Appends an element to this deque, returning a reference to it.
    ///
    /// See [`VecDeque::push_back_mut`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut d = ArrDeque::<_, 30>::from([1, 2, 3]);
    /// let x = d.push_back_mut(9);
    /// *x += 1;
    /// assert_eq!(d.back(), Some(&10));
    /// ```
    #[must_use = msg::must_use_reference!("ArrDeque::push_back")]
    pub fn push_back_mut(&mut self, value: T) -> &mut T {
        assert!(self.len() < N, msg::cap_over_addition!(), N);
        self.insert_mut(self.len(), value)
    }

    /// Removes and returns an element at given `index`.
    ///
    /// See [`VecDeque::remove`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// assert_eq!(buf, ['a', 'b', 'c']);
    ///
    /// assert_eq!(buf.remove(1), Some('b'));
    /// assert_eq!(buf, ['a', 'c']);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }

        let removing = unsafe { self.copy_buf_val(self.to_buf_idx(index)) };
        self.clear_range(&(index..=index), false);
        Some(removing)
    }

    /// Inserts an element at given index.
    ///
    /// See [`VecDeque::insert`].
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Same situations in [`VecDeque::insert`].
    /// - The number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut arr_deque = ArrDeque::<_, 30>::new();
    /// arr_deque.push_back('a');
    /// arr_deque.push_back('b');
    /// arr_deque.push_back('c');
    /// assert_eq!(arr_deque, &['a', 'b', 'c']);
    ///
    /// arr_deque.insert(1, 'd');
    /// assert_eq!(arr_deque, &['a', 'd', 'b', 'c']);
    ///
    /// arr_deque.insert(4, 'e');
    /// assert_eq!(arr_deque, &['a', 'd', 'b', 'c', 'e']);
    /// ```
    pub fn insert(&mut self, index: usize, value: T) {
        let _ = self.insert_mut(index, value);
    }

    /// Inserts an element at given index, returning a reference to it.
    ///
    /// See [`VecDeque::insert_mut`].
    ///
    /// # Panics
    ///
    /// Panics if any of the following cases occured.
    ///
    /// - Same situations in [`VecDeque::insert_mut`].
    /// - The number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut arr_deque = ArrDeque::<_, 30>::from([1, 2, 3]);
    ///
    /// let x = arr_deque.insert_mut(1, 5);
    /// *x += 7;
    /// assert_eq!(arr_deque, &[1, 12, 2, 3]);
    /// ```
    #[must_use = msg::must_use_reference!("ArrDeque::insert")]
    pub fn insert_mut(&mut self, index: usize, value: T) -> &mut T {
        assert!(self.len() < N, msg::cap_over_addition!(), N);
        assert!(index <= self.len(), msg::index_ob!(), index, self.len());

        // Slide slim side range.
        let slim_side = Dir::dec(index <= self.len - index);
        let slim_range = &self.side_range(index, slim_side);
        let offset = Offset::new(slim_side, 1);
        self.slide_range(slim_range, offset);

        // Adjust fields.
        let bgn_dec = slim_side.not().binary();
        self.start = util::sub_mod(self.start, bgn_dec, N);
        self.len += 1;

        // Insert value.
        unsafe {
            self.write_buf_val(self.to_buf_idx(index), value);
        }

        // Return reference.
        self.get_mut(index).unwrap()
    }

    /// Swaps elements at two given indices.
    ///
    /// See [`VecDeque::swap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(3);
    /// buf.push_back(4);
    /// buf.push_back(5);
    /// assert_eq!(buf, [3, 4, 5]);
    /// buf.swap(0, 2);
    /// assert_eq!(buf, [5, 4, 3]);
    /// ```
    pub fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.len && j < self.len);
        if i != j {
            let x_ptr = self.buf[self.to_buf_idx(i)].as_mut_ptr();
            let y_ptr = self.buf[self.to_buf_idx(j)].as_mut_ptr();
            unsafe {
                ptr::swap(x_ptr, y_ptr);
            }
        }
    }

    /// Removes element at given index with replacing it with first element.
    ///
    /// See [`VecDeque::swap_remove_front`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// assert_eq!(buf.swap_remove_front(0), None);
    /// buf.push_back(1);
    /// buf.push_back(2);
    /// buf.push_back(3);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// assert_eq!(buf.swap_remove_front(2), Some(3));
    /// assert_eq!(buf, [2, 1]);
    /// ```
    pub fn swap_remove_front(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        self.swap(index, 0);
        self.remove(0)
    }

    /// Removes element at given index with replacing it with last element.
    ///
    /// See [`VecDeque::swap_remove_back`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// assert_eq!(buf.swap_remove_back(0), None);
    /// buf.push_back(1);
    /// buf.push_back(2);
    /// buf.push_back(3);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// assert_eq!(buf.swap_remove_back(0), Some(1));
    /// assert_eq!(buf, [3, 2]);
    /// ```
    pub fn swap_remove_back(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        self.swap(index, self.len() - 1);
        self.remove(self.len() - 1)
    }

    /// Rotates the deque `n` places to the left.
    ///
    /// See [`VecDeque::rotate_left`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf: ArrDeque<_, 30> = (0..10).collect();
    ///
    /// buf.rotate_left(3);
    /// assert_eq!(buf, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    ///
    /// for i in 1..10 {
    ///     assert_eq!(i * 3 % 10, buf[0]);
    ///     buf.rotate_left(3);
    /// }
    /// assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn rotate_left(&mut self, n: usize) {
        assert!(n <= self.len);
        self.rotate(Offset::new(Dir::Dec, n));
    }

    /// Rotates the deque `n` places to the right.
    ///
    /// See [`VecDeque::rotate_right`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf: ArrDeque<_, 30> = (0..10).collect();
    ///
    /// buf.rotate_right(3);
    /// assert_eq!(buf, [7, 8, 9, 0, 1, 2, 3, 4, 5, 6]);
    ///
    /// for i in 1..10 {
    ///     assert_eq!(0, buf[i * 3 % 10]);
    ///     buf.rotate_right(3);
    /// }
    /// assert_eq!(buf, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn rotate_right(&mut self, n: usize) {
        assert!(n <= self.len);
        self.rotate(Offset::new(Dir::Inc, n));
    }

    /// Shortens the deque, keeping the first len elements and dropping the rest.
    ///
    /// See [`VecDeque::truncate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(5);
    /// buf.push_back(10);
    /// buf.push_back(15);
    /// assert_eq!(buf, [5, 10, 15]);
    /// buf.truncate(1);
    /// assert_eq!(buf, [5]);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        if self.len > len {
            drop(self.drain(len..));
        }
    }

    /// Splits the deque into two at the given index.
    ///
    /// See [`VecDeque::split_off`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf: ArrDeque<_, 30> = ['a', 'b', 'c'].into();
    /// let buf2 = buf.split_off(1);
    /// assert_eq!(buf, ['a']);
    /// assert_eq!(buf2, ['b', 'c'])
    /// ```
    #[must_use = msg::must_use_split_off!()]
    pub fn split_off(&mut self, at: usize) -> Self {
        // Collects source slices of return value.
        let src_slices = self.to_slices(&(at..self.len()));

        // Creates destination slices of return value.
        let mut other = Self::new();
        other.len = self.len - at;
        let dst_slices = &mut other.buf[0..other.len];
        let dst_slices = <[_; _]>::from(dst_slices.split_at_mut(src_slices[0].len()));

        // Copy values from source to destination.
        for i in 0..dst_slices.len() {
            unsafe {
                let copy_len = dst_slices[i].len();
                let dst_ptr = dst_slices[i].as_mut_ptr();
                let dst_ptr = mem::transmute::<*mut MaybeUninit<T>, *mut T>(dst_ptr);
                let src_ptr = src_slices[i].as_ptr();
                ptr::copy_nonoverlapping(src_ptr, dst_ptr, copy_len);
            }
        }

        // Shrink the length by amount of moved.
        self.len = at;

        // Retuns deque that contains copied values.
        other
    }

    /// Move all the elements of other into this deque.
    ///
    /// See [`VecDeque::append`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let deque: ArrDeque<_, 30> = [1, 2, 3, 3, 5, 6, 7].into();
    /// let i = deque.partition_point(|&x| x < 5);
    ///
    /// assert_eq!(i, 4);
    /// assert!(deque.iter().take(i).all(|&x| x < 5));
    /// assert!(deque.iter().skip(i).all(|&x| !(x < 5)));
    /// ```
    ///
    /// If you want to insert an item to a sorted deque, while maintaining sort order:
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<_, 30> = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55].into();
    /// let num = 42;
    /// let idx = deque.partition_point(|&x| x < num);
    /// deque.insert(idx, num);
    /// assert_eq!(deque, &[0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);
    /// ```
    pub fn append(&mut self, other: &mut Self) {
        assert!(
            self.len()
                .checked_add(other.len())
                .is_some_and(|x| x <= self.capacity()),
            msg::cap_over_addition!(),
            self.capacity(),
        );

        self.extend(other.drain(..));
    }

    /// Resizes the deque.
    ///
    /// See [`VecDeque::resize`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(5);
    /// buf.push_back(10);
    /// buf.push_back(15);
    /// assert_eq!(buf, [5, 10, 15]);
    ///
    /// buf.resize(2, 0);
    /// assert_eq!(buf, [5, 10]);
    ///
    /// buf.resize(5, 20);
    /// assert_eq!(buf, [5, 10, 20, 20, 20]);
    /// ```
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        assert!(new_len <= N, msg::cap_over_resize!(), N, new_len);
        self.resize_with(new_len, || value.clone());
    }

    /// Resizes the deque with generator.
    ///
    /// See [`VecDeque::resize_with`].
    ///
    /// # Panics
    ///
    /// Panics if the number of elements exceeds the capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.push_back(5);
    /// buf.push_back(10);
    /// buf.push_back(15);
    /// assert_eq!(buf, [5, 10, 15]);
    ///
    /// buf.resize_with(5, Default::default);
    /// assert_eq!(buf, [5, 10, 15, 0, 0]);
    ///
    /// buf.resize_with(2, || unreachable!());
    /// assert_eq!(buf, [5, 10]);
    ///
    /// let mut state = 100;
    /// buf.resize_with(5, || { state += 1; state });
    /// assert_eq!(buf, [5, 10, 101, 102, 103]);
    /// ```
    pub fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> T) {
        assert!(new_len <= N, msg::cap_over_resize!(), N, new_len);
        let missing_len = new_len.saturating_sub(self.len());
        self.truncate(new_len);
        self.extend(core::iter::repeat_with(generator).take(missing_len));
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// See [`VecDeque::retain`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.extend(1..5);
    /// buf.retain(|&x| x % 2 == 0);
    /// assert_eq!(buf, [2, 4]);
    /// ```
    ///
    /// Because the elements are visited exactly once in the original order,
    /// external state may be used to decide which elements to keep.
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.extend(1..6);
    ///
    /// let keep = [false, true, true, false, true];
    /// let mut iter = keep.iter();
    /// buf.retain(|_| *iter.next().unwrap());
    /// assert_eq!(buf, [2, 3, 5]);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain_mut(|x| f(x));
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// See [`VecDeque::retain`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    /// buf.extend(1..5);
    /// buf.retain_mut(|x| if *x % 2 == 0 {
    ///     *x += 1;
    ///     true
    /// } else {
    ///     false
    /// });
    /// assert_eq!(buf, [3, 5]);
    /// ```
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut count = 0;
        for i in self.all() {
            if f(&mut self[i]) {
                self.swap(count, i);
                count += 1;
            }
        }

        self.truncate(count);
    }

    /// Removes elements in the given range by returned iterataor.
    ///
    /// See [`VecDeque::drain`].
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut deque: ArrDeque<_, 30> = [1, 2, 3].into();
    /// let drained = deque.drain(2..).collect::<ArrDeque<_, 30>>();
    /// assert_eq!(drained, [3]);
    /// assert_eq!(deque, [1, 2]);
    ///
    /// // A full range clears all contents, like `clear()` does
    /// deque.drain(..);
    /// assert!(deque.is_empty());
    /// ```
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T, N>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();
        Drain::new(self, util::slice_range(range, ..len))
    }

    /// Rearranges the internal storage to one contiguous slice.
    ///
    /// See [`VecDeque::make_contiguous`].
    ///
    /// # Examples
    ///
    /// Sorting the content of a deque.
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 15>::new();
    ///
    /// buf.push_back(2);
    /// buf.push_back(1);
    /// buf.push_front(3);
    ///
    /// // sorting the deque
    /// buf.make_contiguous().sort();
    /// assert_eq!(buf.as_slices(), (&[1, 2, 3] as &[_], &[] as &[_]));
    ///
    /// // sorting it in reverse order
    /// buf.make_contiguous().sort_by(|a, b| b.cmp(a));
    /// assert_eq!(buf.as_slices(), (&[3, 2, 1] as &[_], &[] as &[_]));
    /// ```
    ///
    /// Getting immutable access to the contiguous slice.
    ///
    /// ```
    /// use arr_deque::prelude::*;
    ///
    /// let mut buf = ArrDeque::<_, 30>::new();
    ///
    /// buf.push_back(2);
    /// buf.push_back(1);
    /// buf.push_front(3);
    ///
    /// buf.make_contiguous();
    /// if let (slice, &[]) = buf.as_slices() {
    ///     // we can now be sure that `slice` contains all elements of the deque,
    ///     // while still having immutable access to `buf`.
    ///     assert_eq!(buf.len(), slice.len());
    ///     assert_eq!(slice, &[3, 2, 1] as &[_]);
    /// }
    /// ```
    pub fn make_contiguous(&mut self) -> &mut [T] {
        self.adjust_ring_start(0);
        self.as_mut_slices().0
    }
}

/// Original methods.
impl<T, const N: usize> ArrDeque<T, N> {
    /// Adjust position of ring buffer head.
    ///
    /// # Panics
    ///
    /// Panics if `pos` is greater or equal than `N`.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_deque::ArrDeque;
    ///
    /// let deque = &mut ArrDeque::<_, 7>::from_iter([0, 1, 2, 3, 4]);
    /// deque.adjust_ring_start(4);
    /// assert_eq!(deque.as_slices().0, &[0, 1, 2]);
    /// assert_eq!(deque.as_slices().1, &[3, 4]);
    /// ```
    pub fn adjust_ring_start(&mut self, pos: usize) {
        assert!(pos < N);
        let ranges = self.to_buf_ranges(&self.all());
        let src_fst_len = ranges[0].len();
        let dst_fst_len = (N - pos).min(self.len);
        let inc_fst_len = Offset::measure(src_fst_len, dst_fst_len);

        // Join two range.
        if !ranges[1].is_empty() {
            self.move_range(&ranges[0], ranges[1].end);
        }

        // Rotate joined range.
        let joined = ranges[1].start..ranges[0].end;
        util::rotate_arr(&mut self.buf[joined.clone()], inc_fst_len.flip());

        // Copy two range part.
        let fst_range = (joined.end - dst_fst_len)..joined.end;
        let snd_range = joined.start..(joined.start + self.len - dst_fst_len);
        self.move_range(&fst_range, pos);
        self.move_range(&snd_range, 0);

        // Update start position.
        self.start = pos;
    }
}

/// Crate public methods.
impl<T, const N: usize> ArrDeque<T, N> {
    /// Returns an item.
    ///
    /// # Safety
    ///
    /// This methods copy value. So be careful about aliasing rules.
    pub(crate) unsafe fn copy_val(&self, index: DeqIdx) -> T {
        unsafe { self.copy_buf_val(self.to_buf_idx(index)) }
    }

    /// Clear elements in given range.
    pub(crate) fn clear_range<R>(&mut self, range: &R, drop: bool)
    where
        R: RangeBounds<usize>,
    {
        let rx = &self.all();
        let ry = &util::range_cap(range, &..self.len);
        let target = &util::range_prod(rx, ry);
        let rests = &util::range_diff(rx, ry);
        let slim_dir = Dir::dec(rests[0].len() <= rests[1].len());
        let slim_range = &rests[slim_dir.binary()];
        let start_offset = slim_dir.not().binary() * target.len();

        // Drop items.
        if drop {
            for i in target.clone() {
                let buf_idx = self.to_buf_idx(i);
                let item = unsafe { self.copy_buf_val(buf_idx) };
                mem::drop(item);
            }
        }

        // Slide slim range.
        let offset = Offset::new(!slim_dir, target.len());
        self.slide_range(slim_range, offset);

        // Adjust fields.
        self.len -= target.len();
        self.start = util::add_mod(self.start, start_offset, N);
    }
}

/// Private methods.
impl<T, const N: usize> ArrDeque<T, N> {
    /// Returns all range.
    fn all(&self) -> Range<DeqIdx> {
        0..self.len()
    }

    /// Returns wrap index.
    fn wrap_index(&self) -> DeqIdx {
        N - self.start
    }

    /// Returns range in the given side of the given index.
    fn side_range(&self, index: DeqIdx, dir: Dir) -> Range<DeqIdx> {
        let inc_side = 0..index;
        let dec_side = index..self.len();
        if dir.is_dec() { inc_side } else { dec_side }
    }

    /// Returns physical index from the given logical index.
    fn to_buf_idx(&self, index: DeqIdx) -> BufIdx {
        util::add_mod(self.start, index, N)
    }

    /// Returns one physical ranges from one logical range.
    fn to_buf_range(&self, range: &Range<DeqIdx>) -> Range<BufIdx> {
        let s = self.to_buf_idx(range.start);
        s..s + range.len()
    }

    /// Returns two physical ranges from one logical range.
    fn to_buf_ranges(&self, range: &Range<DeqIdx>) -> [Range<BufIdx>; 2] {
        let ranges = util::range_cut(range, self.wrap_index());
        ranges.map(|r| self.to_buf_range(&r))
    }

    /// Returns two slices from one logical range.
    fn to_slices(&self, range: &Range<DeqIdx>) -> [&[T]; 2] {
        let [r1, r2] = self.to_buf_ranges(range);
        let [s1, s2] = [&self.buf[r1], &self.buf[r2]];
        let s1 = unsafe { mem::transmute::<&[MaybeUninit<T>], &[T]>(s1) };
        let s2 = unsafe { mem::transmute::<&[MaybeUninit<T>], &[T]>(s2) };
        [s1, s2]
    }

    /// Returns two mutable slices from one logical range.
    fn to_slices_mut(&mut self, range: &Range<DeqIdx>) -> [&mut [T]; 2] {
        let [r1, r2] = self.to_buf_ranges(range);
        let [s1, s2] = self.buf.get_disjoint_mut([r1, r2]).unwrap();
        let s1 = unsafe { mem::transmute::<&mut [MaybeUninit<T>], &mut [T]>(s1) };
        let s2 = unsafe { mem::transmute::<&mut [MaybeUninit<T>], &mut [T]>(s2) };
        [s1, s2]
    }

    /// Rotate deque.
    fn rotate(&mut self, offset: Offset) {
        if offset.len() == 0 {
            return;
        }

        // Calculate rotation size.
        let rotation_len_fwd = offset.len();
        let rotation_len_bwd = self.len - offset.len();
        let use_fwd = rotation_len_fwd <= rotation_len_bwd;
        let rotation_len = rotation_len_fwd.min(rotation_len_bwd);

        // Calculate start position of range to work.
        let work_range_start = match (offset.dir(), use_fwd) {
            (Dir::Dec, true) => 0,
            (Dir::Inc, false) => 0,
            (Dir::Dec, false) => rotation_len_fwd,
            (Dir::Inc, true) => rotation_len_bwd,
        };

        // Calculate update informations.
        let work_range = work_range_start..(work_range_start + rotation_len);
        let work_offset_dir = S(offset.dir()).map_if(!use_fwd, Dir::not);
        let start_offset_dir = S(offset.dir()).map_if(use_fwd, Dir::not);
        let work_offset = Offset::new(work_offset_dir, N - self.len);
        let start_offset = Offset::new(start_offset_dir, rotation_len);

        // Slide range and adjust start position.
        self.slide_range(&work_range, work_offset);
        self.start = util::offset_mod(self.start, start_offset, N);
    }

    /// Slides values in range.
    fn slide_range(&mut self, range: &Range<DeqIdx>, offset: Offset) {
        debug_assert!(range.len() + offset.len() <= N);
        let inc = offset.is_inc();
        let len = offset.len();

        // Separate target range.
        let cut_pos = if inc { N - len } else { len };
        let ranges = &mut self.to_buf_ranges(range).into_iter();
        let ranges = &mut ranges.flat_map(|r| util::range_cut(&r, cut_pos));
        let ranges = array::from_fn::<_, 4, _>(|_| ranges.next().unwrap());

        // Slide values in each ranges.
        for range in S(ranges).upget_if(inc, |x| x.reverse()) {
            let dst_start = util::offset_mod(range.start, offset, N);
            self.move_range(&range, dst_start);
        }
    }

    /// Moves values in range.
    fn move_range(&mut self, src_range: &Range<BufIdx>, dst_start: usize) {
        let dst_range = dst_start..(dst_start + src_range.len());
        let dst = self.buf[dst_range].as_mut_ptr();
        let src = self.buf[src_range.clone()].as_ptr();
        unsafe {
            ptr::copy(src, dst, src_range.len());
        }
    }

    /// Returns value at given index.
    ///
    /// # Safety
    ///
    /// This methods copy value. So be careful about aliasing rules.
    unsafe fn copy_buf_val(&self, index: BufIdx) -> T {
        unsafe { (self.buf.as_ptr().add(index) as *const T).read() }
    }

    /// Write value to index.
    ///
    /// # Safety
    ///
    /// This methods delete existing value without destructor.
    unsafe fn write_buf_val(&mut self, index: BufIdx, value: T) {
        let value = MaybeUninit::new(value);
        unsafe {
            ((&mut self.buf[index]) as *mut MaybeUninit<T>).write(value);
        }
    }
}

impl<T, const N: usize> Clone for ArrDeque<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut ret = Self::new();
        ret.start = self.start;
        ret.len = self.len;
        unsafe {
            let (dst1, dst2) = ret.as_mut_slices();
            let (src1, src2) = self.as_slices();
            ptr::copy_nonoverlapping(src1.as_ptr(), dst1.as_mut_ptr(), dst1.len());
            ptr::copy_nonoverlapping(src2.as_ptr(), dst2.as_mut_ptr(), dst2.len());
        }

        ret
    }
}

impl<T, const N: usize> Debug for ArrDeque<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, const N: usize> Default for ArrDeque<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for ArrDeque<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Eq for ArrDeque<T, N>
where
    T: Eq,
{
    // nop.
}

/// # Notes
///
/// Some methods panic if the number of elements exceeds the capacity.
impl<T, const N: usize> Extend<T> for ArrDeque<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

/// # Notes
///
/// Some methods panic if the number of elements exceeds the capacity.
impl<'a, T: 'a, const N: usize> Extend<&'a T> for ArrDeque<T, N>
where
    T: Copy,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

/// # Notes
///
/// Some methods panic if the number of elements exceeds the capacity.
impl<T, const N: usize, const L: usize> From<[T; L]> for ArrDeque<T, N> {
    fn from(value: [T; L]) -> Self {
        assert!(L <= N);
        value.into_iter().collect()
    }
}

/// # Notes
///
/// Some methods panic if the number of elements exceeds the capacity.
#[cfg(feature = "alloc")]
impl<T, const N: usize> From<Vec<T>> for ArrDeque<T, N> {
    fn from(value: Vec<T>) -> Self {
        assert!(value.len() <= N);
        value.into_iter().collect()
    }
}

/// # Notes
///
/// Some methods panic if the number of elements exceeds the capacity.
impl<T, const N: usize> FromIterator<T> for ArrDeque<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut ret = Self::new();
        ret.extend(iter);
        ret
    }
}

impl<T, const N: usize> Hash for ArrDeque<T, N>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for item in self.iter() {
            item.hash(state);
        }
    }
}

impl<T, const N: usize> Index<usize> for ArrDeque<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), msg::index_ob!(), index, self.len());
        self.get(index).unwrap()
    }
}

impl<T, const N: usize> IndexMut<usize> for ArrDeque<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), msg::index_ob!(), index, self.len());
        self.get_mut(index).unwrap()
    }
}

impl<T, const N: usize> IntoIterator for ArrDeque<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;
    fn into_iter(self) -> IntoIter<T, N> {
        IntoIter::new(self)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a ArrDeque<T, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut ArrDeque<T, N> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

impl<T, const N: usize> Ord for ArrDeque<T, N>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T, const N: usize> PartialEq for ArrDeque<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

#[cfg(feature = "alloc")]
impl<T, U, const N: usize> PartialEq<Vec<U>> for ArrDeque<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Vec<U>) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, U, const N: usize> PartialEq<&[U]> for ArrDeque<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, U, const N: usize> PartialEq<&mut [U]> for ArrDeque<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut [U]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, U, const N: usize, const M: usize> PartialEq<[U; M]> for ArrDeque<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U; M]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, U, const N: usize, const M: usize> PartialEq<&[U; M]> for ArrDeque<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&[U; M]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, U, const N: usize, const M: usize> PartialEq<&mut [U; M]> for ArrDeque<T, N>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &&mut [U; M]) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T, const N: usize> PartialOrd for ArrDeque<T, N>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

#[cfg(feature = "alloc")]
impl<T, const N: usize> From<ArrDeque<T, N>> for Vec<T> {
    fn from(value: ArrDeque<T, N>) -> Self {
        value.into_iter().collect()
    }
}

impl<const N: usize> BufRead for ArrDeque<u8, N> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        let (slice_this_time, _) = self.as_slices();
        Ok(slice_this_time)
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        assert!(amt <= self.len());
        self.clear_range(&..amt, false);
    }
}

impl<const N: usize> Read for ArrDeque<u8, N> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let (ref mut slice_this_time, _) = self.as_slices();
        let count = Read::read(slice_this_time, buf)?;
        self.clear_range(&..count, false);
        Ok(count)
    }
}

/// # Notes
///
/// Some methods panic if the number of elements exceeds the capacity.
impl<const N: usize> Write for ArrDeque<u8, N> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        assert!(self.len() + buf.len() <= N);
        self.extend(buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
