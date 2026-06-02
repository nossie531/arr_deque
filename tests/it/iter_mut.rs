use crate::for_test::*;
use arr_deque::*;

#[test]
fn fmt() {
    for mut builder in ts::iters::all() {
        let target = builder.build_iter_mut();
        let result = format!("{:?}", target);
        assert!(!result.is_empty());
    }
}

#[test]
fn default() {
    let result = IterMut::<i32>::default();
    assert_eq!(result.count(), 0);
}

#[test]
fn next() {
    for builder in ts::iters::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = &mut builder.0.build_iter_mut();
        let master = &mut builder.1.build_master_iter_mut();

        // Act.
        let asis = target.next();
        let tobe = master.next();

        // Assert value.
        assert_eq!(asis, tobe);

        // Assert reference position.
        if let Some(asis) = asis {
            *asis = ts::VAL;
            let pos = builder.0.head_skip();
            assert_eq!(builder.0.deque()[pos], ts::VAL);
        }
    }
}

#[test]
fn nth() {
    for (builder, n) in ts::iters::all().flat_map(ts::IterBuilder::each_index) {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = &mut builder.0.build_iter_mut();
        let master = &mut builder.1.build_master_iter_mut();

        // Act.
        let asis = target.nth(n);
        let tobe = master.nth(n);

        // Assert value.
        assert_eq!(asis, tobe);

        // Assert reference position.
        if let Some(asis) = asis {
            *asis = ts::VAL;
            let pos = builder.0.head_skip() + n;
            assert_eq!(builder.0.deque()[pos], ts::VAL);
        }
    }
}

#[test]
fn size_hint() {
    for builder in ts::iters::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = &mut builder.0.build_iter_mut();
        let master = &mut builder.1.build_master_iter_mut();

        // Act.
        let asis = target.size_hint();
        let tobe = master.size_hint();

        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn count() {
    for builder in ts::iters::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = builder.0.build_iter_mut();
        let master = builder.1.build_master_iter_mut();
        // Act.
        let asis = target.count();
        let tobe = master.count();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn last() {
    for builder in ts::iters::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = builder.0.build_iter_mut();
        let master = builder.1.build_master_iter_mut();

        // Act.
        let asis = target.last();
        let tobe = master.last();

        // Assert value.
        assert_eq!(asis, tobe);

        // Assert reference position.
        if let Some(asis) = asis {
            *asis = ts::VAL;
            let pos = builder.0.deque().len() - builder.0.tail_skip() - 1;
            assert_eq!(builder.0.deque()[pos], ts::VAL);
        }
    }
}

#[test]
fn next_back() {
    for builder in ts::iters::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = &mut builder.0.build_iter_mut();
        let master = &mut builder.1.build_master_iter_mut();

        // Act.
        let asis = target.next_back();
        let tobe = master.next_back();

        // Assert value.
        assert_eq!(asis, tobe);

        // Assert reference position.
        if let Some(asis) = asis {
            *asis = ts::VAL;
            let pos = builder.0.deque().len() - builder.0.tail_skip() - 1;
            assert_eq!(builder.0.deque()[pos], ts::VAL);
        }
    }
}

#[test]
fn nth_back() {
    for (builder, n) in ts::iters::all().flat_map(ts::IterBuilder::each_index) {
        // Arrange.
        let mut builder = tu::dup(builder);
        let target = &mut builder.0.build_iter_mut();
        let master = &mut builder.1.build_master_iter_mut();

        // Act.
        let asis = target.nth_back(n);
        let tobe = master.nth_back(n);

        // Assert value.
        assert_eq!(asis, tobe);

        // Assert reference position.
        if let Some(asis) = asis {
            *asis = ts::VAL;
            let pos = (builder.0.deque().len() - 1) - (builder.0.tail_skip() + n);
            assert_eq!(builder.0.deque()[pos], ts::VAL);
        }
    }
}
