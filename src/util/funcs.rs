//! Utility functions.

use crate::util::*;
use crate::*;
use core::ops::RangeTo;
use core::ops::{Bound, Range, RangeBounds};
use rich_range::prelude::*;

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
    let ranges = rw::refr(range).cut(&pos);
    let rx = ranges.0.unwrap_or_default().try_into().unwrap();
    let ry = ranges.1.unwrap_or_default().try_into().unwrap();
    [rx, ry]
}

/// Returns the product of two ranges.
pub(crate) fn range_prod(rx: &Range<usize>, ry: &Range<usize>) -> Range<usize> {
    (rw::refr(rx) & rw::refr(ry)).0
}

/// Returns the difference of two ranges.
pub(crate) fn range_diff(rx: &Range<usize>, ry: &Range<usize>) -> [Range<usize>; 2] {
    let ranges = rw::refr(rx).diff(rw::refr(ry));
    let ret1 = ranges.0.unwrap_or_default().try_into().unwrap();
    let ret2 = ranges.1.unwrap_or_default().try_into().unwrap();
    [ret1, ret2]
}

/// Returns capped range.
pub(crate) fn range_cap<R>(range: &R, len: usize) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let range = range_cap_raw(range, len);
    range.start.min(range.end)..range.end
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
pub(crate) fn range_slice<R>(range: R, bounds: RangeTo<usize>) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    let range = range_cap_raw(&range, bounds.end);
    if range.start > range.end {
        panic!(msg::range_order_rev!(), range.start, range.end);
    } else if range.end > bounds.end {
        panic!(msg::range_end_gt_bounds_end!(), range.end, bounds.end);
    } else {
        range
    }
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
    let fst_part = n.min(iters[0].len());
    iters[0]
        .nth(fst_part)
        .or_else(|| iters[1].nth(n - fst_part))
}

/// Returns `n`th item of chained iterators from the end.
pub(crate) fn chained_nth_back<I>(iters: [&mut I; 2], n: usize) -> Option<I::Item>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    let fst_part = n.min(iters[1].len());
    iters[1]
        .nth_back(fst_part)
        .or_else(|| iters[0].nth_back(n - fst_part))
}

/// Returns raw capped range.
fn range_cap_raw<R>(range: &R, len: usize) -> Range<usize>
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
        Bound::Unbounded => len,
    };

    s..e
}
