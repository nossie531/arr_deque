macro_rules! grid_iter {
    ($iter1: expr, $iter2: expr) => {
        IntoIterator::into_iter($iter1)
            .flat_map(move |item1| IntoIterator::into_iter($iter2).map(move |item2| (item1, item2)))
    };
}

pub(crate) use grid_iter;
