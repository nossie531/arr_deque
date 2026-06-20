use crate::for_test::*;
use std::iter;
use subject::exts::UpgetExt;
use subject::prelude::*;

pub fn all_for_append_normal() -> impl Iterator<Item = [ts::SampleDeque; 2]> {
    let normal_case = (ts::NORMAL_LEN, (ts::CAPACITY - ts::NORMAL_LEN) / 2);
    let edge_case1 = (ts::CAPACITY, 0);
    let edge_case2 = (0, ts::CAPACITY);
    let edge_case3 = (ts::NORMAL_LEN, ts::CAPACITY - ts::NORMAL_LEN);
    let len_pairs = [normal_case, edge_case1, edge_case2, edge_case3];
    len_pairs.into_iter().map(|(len_x, len_y)| {
        let deque_x = ts::deque::custom_len(len_x);
        let deque_y = ts::deque::custom_len(len_y);
        [deque_x, deque_y]
    })
}

pub fn all_for_append_overflow() -> impl Iterator<Item = [ts::SampleDeque; 2]> {
    const MORE: usize = 5;
    let edge_case1 = (ts::CAPACITY, 1);
    let edge_case2 = (1, ts::CAPACITY);
    let edge_case3 = (ts::NORMAL_LEN, ts::CAPACITY - ts::NORMAL_LEN + 1);
    let normal_case = (ts::NORMAL_LEN, ts::CAPACITY - ts::NORMAL_LEN + MORE);
    let len_pairs = [edge_case1, edge_case2, edge_case3, normal_case];
    len_pairs.into_iter().map(|(len_x, len_y)| {
        let deque_x = ts::deque::custom_len(len_x);
        let deque_y = ts::deque::custom_len(len_y);
        [deque_x, deque_y]
    })
}

pub fn all_for_cmp_normal() -> impl Iterator<Item = [ts::SampleDeque; 2]> {
    all_for_cmp_float()
        .filter(|[x, y]| {
            let x_has_nan = x.iter().any(|e| e.is_nan());
            let y_has_nan = y.iter().any(|e| e.is_nan());
            !x_has_nan && !y_has_nan
        })
        .map(|[x, y]| {
            let x = ts::SampleDeque::from_iter(x.iter().map(|&x| x as ts::Val));
            let y = ts::SampleDeque::from_iter(y.iter().map(|&y| y as ts::Val));
            [x, y]
        })
}

pub fn all_for_cmp_float() -> impl Iterator<Item = [ts::SampleDequeFloat; 2]> {
    let vs_eq = pair_for_cmp_float();

    let vs_large_len = S(pair_for_cmp_float()).upget(|x| {
        x[0].pop_back();
    });

    let vs_small_len = S(pair_for_cmp_float()).upget(|x| {
        x[0].push_back(0.);
    });

    let vs_large_val = ts::indicies_inside(ts::NORMAL_LEN).map(move |i| {
        let mut pair = pair_for_cmp_float();
        pair[0][i] = 0.;
        pair[1][i] = 1.;
        pair
    });

    let vs_small_val = ts::indicies_inside(ts::NORMAL_LEN).map(move |i| {
        let mut pair = pair_for_cmp_float();
        pair[0][i] = 1.;
        pair[1][i] = 0.;
        pair
    });

    iter::empty()
        .chain(iter::once(vs_eq))
        .chain(iter::once(vs_large_len))
        .chain(iter::once(vs_small_len))
        .chain(vs_large_val)
        .chain(vs_small_val)
}

fn pair_for_cmp_float() -> [ts::SampleDequeFloat; 2] {
    let single = ts::deque::type_float();
    [single.clone(), single.clone()]
}
