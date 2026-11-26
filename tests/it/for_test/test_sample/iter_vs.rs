use crate::for_test::*;

pub fn each_index(builder: ts::IterBuilder) -> impl Iterator<Item = (ts::IterBuilder, usize)> {
    let skipped = builder.head_skip() + builder.tail_skip();
    let len = builder.deque().len() - skipped;
    let indices = ts::indices(len);
    indices.map(move |x| (builder.clone(), x))
}
