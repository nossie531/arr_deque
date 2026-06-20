use crate::for_test::*;
use arr_deque::ArrDeque;
use drop_tracer::DropTracer;

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
    // Arrange.
    let deque = &mut ArrDeque::<_, { ts::CAPACITY }>::new();
    let tracer = &DropTracer::new();
    deque.extend((0..ts::NORMAL_LEN).map(|_| tracer.trace(ts::VAL)));
    let mut target = deque.drain(..);
    target.next();
    target.next_back();
    // Act.
    std::mem::drop(target);
    // Assert.
    assert_eq!(tracer.living_count(), 0);
}
