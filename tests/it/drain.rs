use crate::for_test::*;
use drop_tracer::prelude::*;
use std::mem;
use test_panic::prelude::*;

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
    for builder in ts::drains::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
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
    for builder in ts::drains::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
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
    for builder in ts::drains::all() {
        // Arrange.
        let mut builder = tu::dup(builder);
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
    for builder in ts::drains::all_with_bomb() {
        // Arrange environments.
        let mut builder = tu::dup(builder);
        let target_deque = builder.0.target_deque_mut();
        let master_deque = builder.1.master_deque_mut();
        let target_tracer = &DropTracer::new();
        let master_tracer = &DropTracer::new();
        let target_tags = tu::tag_map(target_deque.iter());
        let master_tags = tu::tag_map(master_deque.iter());
        tu::trace_all(target_deque.iter_mut(), target_tracer);
        tu::trace_all(master_deque.iter_mut(), master_tracer);

        // Arrange target.
        let target = builder.0.build_target();
        let master = builder.1.build_master();

        // Act.
        let asis = test_panic(|| mem::drop(target));
        let tobe = test_panic(|| mem::drop(master));

        // Assert.
        let target_deque = builder.0.target_deque();
        let master_deque = builder.1.master_deque();
        assert_eq!(asis, tobe);
        assert!(target_deque.iter().eq(master_deque.iter()));
        assert!(tu::eq_logs(
            &target_tracer,
            &master_tracer,
            &target_tags,
            &master_tags
        ));
    }
}
