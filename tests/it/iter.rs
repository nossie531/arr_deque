use crate::for_test::*;
use arr_deque::*;

#[test]
fn clone() {
    for builder in ts::iters::all() {
        let target = builder.build_iter();
        let result = target.clone();
        assert!(target.eq(result));
    }
}

#[test]
fn fmt() {
    for builder in ts::iters::all() {
        let target = builder.build_iter();
        let result = format!("{:?}", target);
        assert!(!result.is_empty());
    }
}

#[test]
fn default() {
    let result = Iter::<i32>::default();
    assert_eq!(result.count(), 0);
}

#[test]
fn next() {
    for builder in ts::iters::all() {
        // Arrange.
        let target = &mut builder.build_iter();
        let master = &mut builder.build_master_iter();
        // Act.
        let asis = target.next();
        let tobe = master.next();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn nth() {
    for (builder, n) in ts::iters::all().flat_map(ts::IterBuilder::each_index) {
        // Arrange.
        let target = &mut builder.build_iter();
        let master = &mut builder.build_master_iter();
        // Act.
        let asis = target.nth(n);
        let tobe = master.nth(n);
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn size_hint() {
    for builder in ts::iters::all() {
        // Arrange.
        let target = &mut builder.build_iter();
        let master = &mut builder.build_master_iter();
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
        let target = builder.build_iter();
        let master = builder.build_master_iter();
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
        let target = builder.build_iter();
        let master = builder.build_master_iter();
        // Act.
        let asis = target.last();
        let tobe = master.last();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn next_back() {
    for builder in ts::iters::all() {
        // Arrange.
        let target = &mut builder.build_iter();
        let master = &mut builder.build_master_iter();
        // Act.
        let asis = target.next_back();
        let tobe = master.next_back();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn nth_back() {
    for (builder, n) in ts::iters::all().flat_map(ts::IterBuilder::each_index) {
        // Arrange.
        let target = &mut builder.build_iter();
        let master = &mut builder.build_master_iter();
        // Act.
        let asis = target.nth_back(n);
        let tobe = master.nth_back(n);
        // Assert.
        assert_eq!(asis, tobe);
    }
}
