use crate::for_test::*;

pub fn all() -> impl Iterator<Item = ts::DrainBuilder> {
    let deques = ts::deques::all();
    deques
        .map(|x| ts::DrainBuilder::default().with_deque(x))
        .flat_map(each_ranges)
        .flat_map(each_skips)
}

fn each_ranges(builder: ts::DrainBuilder) -> impl Iterator<Item = ts::DrainBuilder> {
    let ranges = ts::ranges_inside(builder.deque().len());
    ranges.map(move |x| builder.clone().with_range(x))
}

fn each_skips(builder: ts::DrainBuilder) -> impl Iterator<Item = ts::DrainBuilder> {
    let range_len = builder.range().len();
    let ranges_in_range = || ts::ranges_inside(builder.range().len());
    let head_skips = ranges_in_range().map(|x| x.start);
    let tail_skips = ranges_in_range().map(move |x| range_len - x.end);
    let skips = head_skips.zip(tail_skips);
    skips.map(move |x| builder.clone().with_head_skip(x.0).with_tail_skip(x.1))
}
