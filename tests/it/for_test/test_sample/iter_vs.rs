use crate::for_test::*;

pub fn each_index(builder: ts::IterBuilder) -> impl Iterator<Item = (ts::IterBuilder, usize)> {
    let skiped = builder.head_skip() + builder.tail_skip();
    let len = builder.deque().len() - skiped;
    let indicies = ts::indicies(len);
    indicies.map(move |x| (builder.clone(), x))
}
