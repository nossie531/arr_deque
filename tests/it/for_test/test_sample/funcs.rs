use crate::for_test::*;
use std::ops::Range;

pub fn indicies(len: usize, max: usize) -> impl Iterator<Item = usize> {
    assert!(len <= max);
    const PLUS_ALPHA: usize = 3;
    let insides = indicies_inside(len);
    let outsides = indicies_outside(len, len + PLUS_ALPHA);
    insides.chain(outsides)
}

pub fn indicies_more(len: usize, capacity: usize) -> impl Iterator<Item = usize> {
    assert!(len <= capacity);
    let insides = indicies_inside(len);
    let outsides = indicies_outside(len, capacity);
    insides.chain(outsides)
}

pub fn indicies_inside(len: usize) -> impl Iterator<Item = usize> {
    const SEPARATE_COUNT: usize = 5;
    let count = if len > 0 { SEPARATE_COUNT } else { 0 };
    (0..count).map(move |i| tu::step_index(i, len, SEPARATE_COUNT))
}

pub fn indicies_outside(len: usize, max: usize) -> impl Iterator<Item = usize> {
    assert!(len <= max);
    let normal_ob_index = len;
    let insert_ob_index = (len + 1).min(max);
    let more_ob_index = max;
    [normal_ob_index, insert_ob_index, more_ob_index].into_iter()
}

pub fn index_pairs(len: usize, max: usize) -> impl Iterator<Item = (usize, usize)> {
    tu::grid_iter!(indicies(len, max), indicies(len, max))
}

pub fn ranges(len: usize) -> impl Iterator<Item = Range<usize>> {
    let insides = ranges_inside(len);
    let outsides = ranges_outside(len);
    insides.chain(outsides)
}

pub fn ranges_inside(len: usize) -> impl Iterator<Item = Range<usize>> {
    let indicies = indicies_inside(len).collect::<Vec<_>>();
    tu::grid_iter!(indicies.clone(), indicies.clone())
        .map(|(s, e)| s..e)
        .filter(|r| !r.is_empty())
        .chain((len == 0).then_some(0..0))
}

pub fn ranges_outside(len: usize) -> impl Iterator<Item = Range<usize>> {
    let broken_range = 1..0;
    let outbound_range = 0..(len + 1);
    [broken_range, outbound_range].into_iter()
}

pub fn predicates() -> impl Iterator<Item = Box<dyn Fn(&i32) -> bool>> {
    {
        let arr: [Box<dyn Fn(&i32) -> bool>; _] = [
            Box::new(|_| true),
            Box::new(|_| false),
            Box::new(|x| *x % 2 == 0),
            Box::new(|x| *x % 2 == 1),
        ];
        arr
    }
    .into_iter()
}

pub fn predicates_mut() -> impl Iterator<Item = Box<dyn Fn(&mut i32) -> bool>> {
    {
        let arr: [Box<dyn Fn(&mut i32) -> bool>; _] = [
            Box::new(|_| true),
            Box::new(|_| false),
            Box::new(|x| *x % 2 == 0),
            Box::new(|x| *x % 2 == 1),
        ];
        arr
    }
    .into_iter()
}

pub fn edit_ref(refr: &mut i32) {
    *refr = refr.wrapping_add(1)
}
