use crate::for_test::*;

pub fn all() -> impl Iterator<Item = ts::IterBuilder> {
    let deques = ts::deques::all();
    deques
        .map(|x| ts::IterBuilder::default().with_deque(x))
        .flat_map(each_skips)
}

fn each_skips(builder: ts::IterBuilder) -> impl Iterator<Item = ts::IterBuilder> {
    let deque_len = builder.deque().len();
    let ranges = || ts::ranges_inside(deque_len);
    let head_skips = ranges().map(|x| x.start);
    let tail_skips = ranges().map(move |x| deque_len - x.end);
    let skips = head_skips.zip(tail_skips);
    skips.map(move |x| builder.clone().with_head_skip(x.0).with_tail_skip(x.1))
}
