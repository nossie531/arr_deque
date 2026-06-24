use crate::for_test::*;
use drop_tracer::DropTracer;
use std::mem;

#[test]
fn fmt() {
    for ref mut builder in ts::drains::all() {
        let target = builder.build_target();
        let result = format!("{:?}", target);
        assert!(!result.is_empty())
    }
}

#[test]
fn next() {
    for ref mut builder in ts::drains::all() {
        // Arrange.
        let mut builder = (builder.clone(), builder.clone());
        let target = &mut builder.0.build_target();
        let master = &mut builder.1.build_master();
        // Act.
        let asis = target.next();
        let tobe = master.next();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn next_back() {
    for ref mut builder in ts::drains::all() {
        // Arrange.
        let mut builder = (builder.clone(), builder.clone());
        let target = &mut builder.0.build_target();
        let master = &mut builder.1.build_master();
        // Act.
        let asis = target.next_back();
        let tobe = master.next_back();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn size_hint() {
    for ref mut builder in ts::drains::all() {
        // Arrange.
        let mut builder = (builder.clone(), builder.clone());
        let target = &mut builder.0.build_target();
        let master = &mut builder.1.build_master();
        // Act.
        let asis = target.size_hint();
        let tobe = master.size_hint();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn drop() {
    for ref mut builder in ts::drains::all_of::<ts::TraceValNt>() {
        // Arrange.
        let tracer = DropTracer::new();
        let numof_remains = builder.deque().len() - builder.range().len();
        let mut target = builder.build_traced_target(&tracer);
        target.next();
        target.next_back();
        // Act.
        mem::drop(target);
        // Assert.
        assert_eq!(tracer.living_count(), numof_remains);
    }
}
