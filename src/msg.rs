//! Messages for this crate.

/// Error message for capacity over.
macro_rules! cap_over_addition {
    () => {
        "Capacity size is {} but more elements were added."
    };
}

/// Error message for capacity over.
macro_rules! cap_over_resize {
    () => {
        "Capacity size is {} but resized to {}."
    };
}

/// Error message for index out of bounds.
macro_rules! index_ob {
    () => {
        "Index out of bounds: index is {} but the length is {}."
    };
}

/// `must_use` attribute message for iterator.
macro_rules! must_use_iter {
    () => {
        "iterators are lazy and do nothing unless consumed"
    };
}

/// `must_use` attribute message for reference.
macro_rules! must_use_reference {
    ($item:expr) => {
        concat!(
            "if you don't need a reference to the value, use `",
            $item,
            "` instead"
        )
    };
}

/// `must_use` attribute message for `split_off`.
macro_rules! must_use_split_off {
    () => {
        "use `.truncate()` if you don't need the other half"
    };
}

/// Error message for cases range end is greater than bounds end.
macro_rules! range_end_gt_bounds_end {
    () => {
        "Range end {} is greater than bounds end {}."
    };
}

/// Error message for cases range start is greater than range end.
macro_rules! range_order_rev {
    () => {
        "Range start {} is greater than end {}."
    };
}

pub(crate) use cap_over_addition;
pub(crate) use cap_over_resize;
pub(crate) use index_ob;
pub(crate) use must_use_iter;
pub(crate) use must_use_reference;
pub(crate) use must_use_split_off;
pub(crate) use range_end_gt_bounds_end;
pub(crate) use range_order_rev;
