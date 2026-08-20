//! Utility functions.

use crate::util::*;
use crate::*;
use core::ops::RangeTo;
use core::ops::{Bound, Range, RangeBounds};
use rich_range::prelude::*;
use subject::exts::UpgetExt;
use subject::prelude::*;

/// Returns offset value modulared by `m`.
pub(crate) fn offset_mod(value: usize, offset: Offset, m: usize) -> usize {
    let flag = offset.dir().is_inc();
    let func = if flag { add_mod } else { sub_mod };
    func(value, offset.len(), m)
}

/// Returns `x + y` value modulared by `m`.
pub(crate) fn add_mod(x: usize, y: usize, m: usize) -> usize {
    let (val, ovf) = x.overflowing_add(y);
    if !ovf {
        val - if val < m { 0 } else { m }
    } else {
        usize::MAX - m + 1 + val
    }
}

/// Returns `x - y` value modulared by `m`.
pub(crate) fn sub_mod(x: usize, y: usize, m: usize) -> usize {
    let (val, ovf) = x.overflowing_sub(y);
    if !ovf {
        val - if val < m { 0 } else { m }
    } else {
        m - (usize::MAX - val + 1)
    }
}

/// Returns the two cutted ranges.
pub(crate) fn range_cut(range: &Range<usize>, pos: usize) -> [Range<usize>; 2] {
    let ranges = <[_; _]>::from(rw::refr(range).cut(&pos));
    let ranges = ranges.map(Option::unwrap_or_default);
    ranges.map(|x| x.try_into().unwrap())
}

/// Returns the product of two ranges.
pub(crate) fn range_prod(rx: &Range<usize>, ry: &Range<usize>) -> Range<usize> {
    (rw::refr(rx) & rw::refr(ry)).0
}

/// Returns the difference of two ranges.
pub(crate) fn range_diff(rx: &Range<usize>, ry: &Range<usize>) -> [Range<usize>; 2] {
    let ranges = rw::refr(rx).diff(rw::refr(ry));
    <[_; _]>::from(ranges).map(|opt| opt.map_or(0..0, |r| r.try_into().unwrap()))
}

/// Returns capped range.
pub(crate) fn range_cap<R>(range: &R, bounds: &RangeTo<usize>) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let rx = rv::new(range).to_univ();
    let ry = rv::new(bounds).to_univ();
    (rx & ry).to_range()
}

/// Returns rotated array.
pub(crate) fn rotate_arr<T>(arr: &mut [T], offset: Offset) {
    let inc = offset.dir().is_inc();
    let func_r = <[_]>::rotate_right;
    let func_l = <[_]>::rotate_left;
    let func = if inc { func_r } else { func_l };
    func(arr, offset.len());
}

/// Returns `n`th item of chained iterators.
pub(crate) fn chained_nth<I>(iters: [&mut I; 2], n: usize) -> Option<I::Item>
where
    I: ExactSizeIterator,
{
    let mut rest = n;
    for iter in iters {
        let progress = rest.min(iter.len());
        let new_rest = rest - progress;
        if let Some(val) = iter.nth(progress) {
            return Some(val);
        }

        rest = new_rest;
    }

    None
}

/// Returns `n`th item of chained iterators from the end.
pub(crate) fn chained_nth_back<I>(iters: [&mut I; 2], n: usize) -> Option<I::Item>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    let mut rest = n;
    for iter in S(iters).upget(|x| x.reverse()) {
        let progress = rest.min(iter.len());
        let new_rest = rest - progress;
        if let Some(val) = iter.nth_back(progress) {
            return Some(val);
        }

        rest = new_rest;
    }

    None
}

/// Performs bounds checking of a range.
///
/// # Notes
///
/// This function is substitute for nightly-only [`range`](std::slice::range).
///
/// # Panics
///
/// Panics in the following cases.
///
/// - Range start and end is reverse order
/// - Range end is greater than bounds end
#[track_caller]
pub(crate) fn slice_range<R>(range: R, bounds: RangeTo<usize>) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let s = match range.start_bound() {
        Bound::Included(x) => *x,
        Bound::Excluded(x) => *x + 1,
        Bound::Unbounded => 0,
    };
    let e = match range.end_bound() {
        Bound::Included(x) => *x + 1,
        Bound::Excluded(x) => *x,
        Bound::Unbounded => bounds.end,
    };

    if s > e {
        panic!(msg::range_order_rev!(), s, e);
    } else if e > bounds.end {
        panic!(msg::range_end_gt_bounds_end!(), e, bounds.end);
    } else {
        s..e
    }
}
