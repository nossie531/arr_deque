use crate::for_test::*;
use arr_deque::prelude::*;
use drop_tracer::prelude::*;
use std::io::{BufRead, Read, Write};
use std::ops::{Index, IndexMut};
use std::{iter, mem};
use test_panic::TestPanicResult;
use test_panic::prelude::*;

#[test]
fn new() {
    let result = ts::SampleDeque::new();
    assert_eq!(result.len(), 0);
    assert_eq!(result.capacity(), ts::CAPACITY);
}

#[test]
fn is_empty() {
    when_empty();
    when_any();

    fn when_empty() {
        let target = ts::deque::empty();
        let result = target.is_empty();
        assert!(result);
    }

    fn when_any() {
        let target = ts::deque::normal();
        let result = target.is_empty();
        assert!(!result);
    }
}

#[test]
fn contains() {
    let targets = ts::deques::all_for_contains();
    let patterns = ts::deque_vs::each_contains_keys;
    for (target, key) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(&target);
        // Act.
        let asis = target.contains(&key);
        let tobe = master.contains(&key);
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn capacity() {
    let target = ts::deque::normal();
    let result = target.capacity();
    assert_eq!(result, ts::CAPACITY);
}

#[test]
fn len() {
    let target = ts::deque::normal();
    let master = ch::vec_deque(&target);
    let asis = target.len();
    let tobe = master.len();
    assert_eq!(asis, tobe);
}

#[test]
fn front() {
    for ref target in ts::deques::all() {
        // Arrange.
        let master = &ch::vec_deque(target);
        // Act.
        let asis = target.front();
        let tobe = master.front();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn front_mut() {
    for ref mut target in ts::deques::all() {
        // Arrange.
        let master = &mut ch::vec_deque(target);
        // Act.
        let asis = target.front_mut();
        let tobe = master.front_mut();
        // Assert.
        assert_eq!(asis, tobe);
        asis.is_some().then(|| ts::edit_ref(asis.unwrap()));
        tobe.is_some().then(|| ts::edit_ref(tobe.unwrap()));
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn back() {
    for ref target in ts::deques::all() {
        // Arrange.
        let master = &ch::vec_deque(target);
        // Act.
        let asis = target.back();
        let tobe = master.back();
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn back_mut() {
    for ref mut target in ts::deques::all() {
        // Arrange.
        let master = &mut ch::vec_deque(target);
        // Act.
        let asis = target.back_mut();
        let tobe = master.back_mut();
        // Assert.
        assert_eq!(asis, tobe);
        asis.is_some().then(|| ts::edit_ref(asis.unwrap()));
        tobe.is_some().then(|| ts::edit_ref(tobe.unwrap()));
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn get() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = ch::vec_deque(target);
        // Act.
        let asis = target.get(index);
        let tobe = master.get(index);
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn get_mut() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref mut target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(&target);
        // Act.
        let asis = target.get_mut(index);
        let tobe = master.get_mut(index);
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn as_slices() {
    for ref mut target in ts::deques::all() {
        // Arrange.
        let master = &mut ch::vec_deque(&target);
        // Act.
        let asis = target.as_slices();
        let tobe = master.as_slices();
        // Assert.
        let asis = asis.0.iter().chain(asis.1);
        let tobe = tobe.0.iter().chain(tobe.1);
        assert!(asis.eq(tobe));
    }
}

#[test]
fn as_mut_slices() {
    for ref mut target in ts::deques::all() {
        // Arrange.
        let master = &mut ch::vec_deque(&target);
        // Act.
        let asis = target.as_mut_slices();
        let tobe = master.as_mut_slices();
        // Assert.
        let asis = asis.0.iter_mut().chain(asis.1);
        let tobe = tobe.0.iter_mut().chain(tobe.1);
        assert!(asis.eq(tobe));
    }
}

#[test]
fn iter() {
    for ref target in ts::deques::all() {
        let master = &ch::vec_deque(&target);
        let asis = target.iter();
        let tobe = master.iter();
        assert!(asis.eq(tobe));
    }
}

#[test]
fn iter_mut() {
    for ref mut target in ts::deques::all() {
        let master = &mut ch::vec_deque(&target);
        let asis = target.iter_mut();
        let tobe = master.iter_mut();
        assert!(asis.eq(tobe));
    }
}

#[test]
fn range() {
    let targets = ts::deques::all_for_binary_search();
    let patterns = ts::deque_vs::each_ranges;
    for (ref target, range) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(target);
        // Act.
        let asis = test_panic(|| target.range(range.clone()));
        let tobe = test_panic(|| master.range(range.clone()));
        // Assert.
        assert_eq!(asis.is_cool(), tobe.is_cool());
        assert!(asis.is_panic() || asis.cool().unwrap().eq(tobe.cool().unwrap()))
    }
}

#[test]
fn range_mut() {
    let targets = ts::deques::all_for_binary_search();
    let patterns = ts::deque_vs::each_ranges;
    for (ref mut target, range) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(target);

        // Act.
        let asis = test_panic(|| target.range_mut(range.clone()));
        let tobe = test_panic(|| master.range_mut(range.clone()));

        // Assert.
        assert_eq!(asis.is_cool(), tobe.is_cool());
        assert!(asis.is_panic() || asis.cool().unwrap().eq(tobe.cool().unwrap()))
    }
}

#[test]
fn partition_point() {
    let targets = ts::deques::all_for_binary_search();
    let patterns = ts::deque_vs::each_search_keys;
    for (target, key) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(&target);
        let pred = |x: &ts::Val| *x < key;
        // Act.
        let asis = target.partition_point(pred);
        let tobe = master.partition_point(pred);
        // Assert.
        assert_eq!(asis, tobe);
    }
}

#[test]
fn binary_search() {
    let targets = ts::deques::all_for_binary_search();
    let patterns = ts::deque_vs::each_search_keys;
    for (target, key) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(&target);

        // Act.
        let asis = target.binary_search(&key);
        let tobe = master.binary_search(&key);

        // Assert.
        // Note: Do not use simple comparing like `assert_eqn!(asis, tobe)`.
        // Because result is ambigous if same values exists in deque.
        assert_eq!(asis.is_ok(), tobe.is_ok());
        assert!(asis.is_err() || target[asis.unwrap()] == key);
    }
}

#[test]
fn binary_search_by() {
    let targets = ts::deques::all_for_binary_search();
    let patterns = ts::deque_vs::each_search_keys;
    for (target, key) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(&target);
        let finder = |x: &ts::Val| x.cmp(&key);

        // Act.
        let asis = target.binary_search_by(finder);
        let tobe = master.binary_search_by(finder);

        // Assert.
        // Note: Do not use simple comparing like `assert_eqn!(asis, tobe)`.
        // Because result is ambigous if same values exists in deque.
        assert_eq!(asis.is_ok(), tobe.is_ok());
        assert!(asis.is_err() || target[asis.unwrap()] == key);
    }
}

#[test]
fn binary_search_by_key() {
    let targets = ts::deques::all_for_binary_search();
    let patterns = ts::deque_vs::each_search_keys;
    for (target, key) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(&target);
        let get_key = |x: &ts::Val| *x;

        // Act.
        let asis = target.binary_search_by_key(&key, get_key);
        let tobe = master.binary_search_by_key(&key, get_key);

        // Assert.
        // Note: Do not use simple comparing like `assert_eqn!(asis, tobe)`.
        // Because result is ambigous if same values exists in deque.
        assert_eq!(asis.is_ok(), tobe.is_ok());
        assert!(asis.is_err() || target[asis.unwrap()] == key);
    }
}

#[test]
fn clear() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let target = &mut ts::deque::normal();
        target.clear();
        assert!(target.is_empty());
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_back(tracer.trace(ts::VAL));
        // Act.
        target.clear();
        // Assert.
        assert_eq!(tracer.living_count(), 0);
    }
}

#[test]
fn pop_front() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        for ref mut target in ts::deques::all() {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.pop_front();
            let tobe = master.pop_front();
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_front(tracer.trace(ts::VAL));
        // Act.
        let result = target.pop_front();
        // Assert.
        assert_eq!(tracer.living_count(), 1);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 0);
    }
}

#[test]
fn pop_back() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        for ref mut target in ts::deques::all() {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.pop_back();
            let tobe = master.pop_back();
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_back(tracer.trace(ts::VAL));
        // Act.
        let result = target.pop_back();
        // Assert.
        assert_eq!(tracer.living_count(), 1);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 0);
    }
}

#[test]
fn pop_front_if() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let targets = ts::deques::all_for_binary_search();
        let patterns = ts::deque_vs::each_predicates_mut;
        for (ref mut target, predicate) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.pop_front_if(|x| predicate(x));
            let tobe = master.pop_front_if(|x| predicate(x));
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_front(tracer.trace(ts::VAL));
        // Act.
        let result = target.pop_front_if(|_| true);
        // Assert.
        assert_eq!(tracer.living_count(), 1);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 0);
    }
}

#[test]
fn pop_back_if() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let targets = ts::deques::all_for_binary_search();
        let patterns = ts::deque_vs::each_predicates_mut;
        for (ref mut target, predicate) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.pop_back_if(|x| predicate(x));
            let tobe = master.pop_back_if(|x| predicate(x));
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_back(tracer.trace(ts::VAL));
        // Act.
        let result = target.pop_back_if(|_| true);
        // Assert.
        assert_eq!(tracer.living_count(), 1);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 0);
    }
}

#[test]
fn push_front() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        for ref mut target in ts::deques::looses() {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            let value = ts::VAL;
            // Act.
            let asis = target.push_front(value);
            let tobe = master.push_front(value);
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        for ref mut target in ts::deques::fulls() {
            let result = test_panic(|| target.push_front(ts::VAL));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn push_front_mut() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        for ref mut target in ts::deques::looses() {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            let value = ts::VAL;
            // Act.
            let asis = target.push_front_mut(value);
            let tobe = master.push_front_mut(value);
            // Assert.
            assert_eq!(asis, tobe);
            ts::edit_ref(asis);
            ts::edit_ref(tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        for ref mut target in ts::deques::fulls() {
            let result = test_panic(|| target.push_front_mut(ts::VAL));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn push_back() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        for ref mut target in ts::deques::looses() {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.push_back(ts::VAL);
            let tobe = master.push_back(ts::VAL);
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        for ref mut target in ts::deques::fulls() {
            let result = test_panic(|| target.push_back(ts::VAL));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn push_back_mut() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        for ref mut target in ts::deques::looses() {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.push_back_mut(ts::VAL);
            let tobe = master.push_back_mut(ts::VAL);
            // Assert.
            assert_eq!(asis, tobe);
            ts::edit_ref(asis);
            ts::edit_ref(tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        for ref mut target in ts::deques::fulls() {
            let result = test_panic(|| target.push_back_mut(ts::VAL));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn remove() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let targets = ts::deques::none_emptys();
        let patterns = ts::deque_vs::each_indicies;
        for (ref mut target, index) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = test_panic(|| target.remove(index));
            let tobe = test_panic(|| master.remove(index));
            // Assert.
            assert_eqn!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_back(tracer.trace(ts::VAL));
        // Act.
        let result = target.remove(0);
        // Assert.
        assert_eq!(tracer.living_count(), 1);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 0);
    }
}

#[test]
fn insert_xxx() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        let targets = ts::deques::looses();
        let patterns = ts::deque_vs::each_indicies;
        for (ref mut target, index) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = test_panic(|| target.insert_mut(index, ts::VAL));
            let tobe = test_panic(|| master.insert_mut(index, ts::VAL));
            // Assert.
            assert_eqn!(asis, tobe);
            asis.is_cool().then(|| ts::edit_ref(asis.cool().unwrap()));
            tobe.is_cool().then(|| ts::edit_ref(tobe.cool().unwrap()));
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        for ref mut target in ts::deques::fulls() {
            let result = test_panic(|| target.insert(0, ts::VAL));
            assert!(result.is_panic());
        }
    }
}

#[test]
fn swap() {
    let targets = ts::deques::looses();
    let patterns = ts::deque_vs::each_index_pairs;
    for (ref mut target, i, j) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(&target);
        // Act.
        let asis = test_panic(|| target.swap(i, j));
        let tobe = test_panic(|| master.swap(i, j));
        // Assert.
        assert_eqn!(asis, tobe);
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn swap_remove_front() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let targets = ts::deques::looses();
        let patterns = ts::deque_vs::each_indicies;
        for (ref mut target, index) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.swap_remove_front(index);
            let tobe = master.swap_remove_front(index);
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_back(tracer.trace(ts::VAL));
        target.push_back(tracer.trace(ts::VAL));
        // Act.
        let result = target.swap_remove_front(1);
        // Assert.
        assert_eq!(tracer.living_count(), 2);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 1);
    }
}

#[test]
fn swap_remove_back() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let targets = ts::deques::looses();
        let patterns = ts::deque_vs::each_indicies;
        for (ref mut target, index) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            // Act.
            let asis = target.swap_remove_back(index);
            let tobe = master.swap_remove_back(index);
            // Assert.
            assert_eq!(asis, tobe);
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.push_back(tracer.trace(ts::VAL));
        target.push_back(tracer.trace(ts::VAL));
        // Act.
        let result = target.swap_remove_back(0);
        // Assert.
        assert_eq!(tracer.living_count(), 2);
        mem::drop(result);
        assert_eq!(tracer.living_count(), 1);
    }
}

#[test]
fn rotate_left() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref mut target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(target);
        // Act.
        let asis = test_panic(|| target.rotate_left(index));
        let tobe = test_panic(|| master.rotate_left(index));
        // Assert.
        assert_eqn!(asis, tobe);
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn rotate_right() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref mut target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(target);
        // Act.
        let asis = test_panic(|| target.rotate_right(index));
        let tobe = test_panic(|| master.rotate_right(index));
        // Assert.
        assert_eqn!(asis, tobe);
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn truncate() {
    when_normal();
    when_impl_drop_for_item();

    fn when_normal() {
        let targets = ts::deques::all();
        let patterns = ts::deque_vs::each_indicies;
        for (ref mut target, index) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(target);
            // Act.
            target.truncate(index);
            master.truncate(index);
            // Assert.
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_impl_drop_for_item() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = &mut ts::deque::empty_of();
        target.extend(iter::repeat_with(|| tracer.trace(ts::VAL)).take(ts::NORMAL_LEN));
        // Act.
        target.truncate(ts::NORMAL_LEN / 2);
        // Assert.
        assert_eq!(tracer.living_count(), ts::NORMAL_LEN / 2);
    }
}

#[test]
fn split_off() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref mut target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(target);
        // Act.
        let asis = test_panic(|| target.split_off(index));
        let tobe = test_panic(|| master.split_off(index));
        // Assert.
        assert_eq!(asis.is_cool(), tobe.is_cool());
        assert!(target.iter().eq(master.iter()));
        assert!(asis.is_panic() || asis.cool().unwrap().iter().eq(tobe.cool().unwrap().iter()));
    }
}

#[test]
fn append() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        let args = ts::deque_pairs::all_for_append_normal();
        for [ref mut target, ref mut other] in args {
            // Arrange.
            let master = &mut ch::vec_deque(target);
            let master_other = &mut ch::vec_deque(other);
            // Act.
            target.append(other);
            master.append(master_other);
            // Assert.
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        let args = ts::deque_pairs::all_for_append_overflow();
        for [ref mut target, ref mut other] in args {
            // Arrange.
            let memo = &target.clone();
            // Act.
            let result = test_panic(|| target.append(other));
            // Assert.
            assert!(result.is_panic());
            assert!((*target).eq(memo));
        }
    }
}

#[test]
fn resize() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        let targets = ts::deques::all();
        let patterns = ts::deque_vs::each_indicies_more;
        for (ref mut target, len) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(target);
            // Act.
            target.resize(len, ts::VAL);
            master.resize(len, ts::VAL);
            // Assert.
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        let target = &mut ts::deque::normal();
        let len = target.capacity() + 1;
        let result = test_panic(|| target.resize(len, ts::VAL));
        assert!(result.is_panic());
    }
}

#[test]
fn resize_with() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        let targets = ts::deques::all();
        let patterns = ts::deque_vs::each_indicies_more;
        for (ref mut target, len) in targets.flat_map(patterns) {
            // Arrange.
            let master = &mut ch::vec_deque(target);
            // Act.
            target.resize_with(len, || ts::VAL);
            master.resize_with(len, || ts::VAL);
            // Assert.
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        let target = &mut ts::deque::normal();
        let len = target.capacity() + 1;
        let result = test_panic(|| target.resize_with(len, || ts::VAL));
        assert!(result.is_panic());
    }
}

#[test]
fn retain_xxx() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_predicates;
    for (ref mut target, f) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(target);
        // Act.
        target.retain(|x| f(x));
        master.retain(|x| f(x));
        // Assert.
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn drain() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_ranges;
    for (ref mut target, range) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(target);

        // Act.
        let asis = test_panic(|| target.drain(range.clone()));
        let tobe = test_panic(|| master.drain(range.clone()));

        // Assert return values to be equal.
        assert!(match asis {
            TestPanicResult::Cool(_) => {
                let asis = asis.cool().unwrap();
                let tobe = tobe.cool().unwrap();
                asis.eq(tobe)
            }
            TestPanicResult::Panic(_) => {
                mem::drop((asis, tobe));
                true
            }
        });

        // Assert deques to be equal.
        assert!(target.iter().eq(master.iter()));
    }
}

#[test]
fn make_contiguous() {
    for ref mut target in ts::deques::all() {
        // Arrange.
        let old = target.clone();

        // Act.
        let result = target.make_contiguous();

        // Assert result values.
        assert!(result.iter().eq(old.iter()));

        // Assert result and target are the same about mutability.
        let edit = |x: ts::Val| x.wrapping_add(1);
        result.iter_mut().for_each(|x| *x = edit(*x));
        assert!(target.iter().cloned().eq(old.iter().map(|x| edit(*x))));

        // Assert side effects to slice of target.
        assert_eq!(target.as_slices().0.len(), old.len());
        assert_eq!(target.as_slices().1.len(), 0);
    }
}

#[test]
fn adjust_ring_start() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        let targets = ts::deques::all();
        let patterns = ts::deque_vs::each_ring_starts;
        for (ref mut target, position) in targets.flat_map(patterns) {
            // Arrange.
            let saved = target.clone();
            // Act.
            target.adjust_ring_start(position);
            // Assert.
            let fst_max = target.capacity() - position;
            let fst_len = target.len().min(fst_max);
            let snd_len = target.len() - fst_len;
            let old_vals = <[_; 2]>::from(saved.as_slices()).into_iter().flatten();
            let new_vals = <[_; 2]>::from(target.as_slices()).into_iter().flatten();
            assert_eq!(target.as_slices().0.len(), fst_len);
            assert_eq!(target.as_slices().1.len(), snd_len);
            assert!(old_vals.eq(new_vals));
        }
    }

    fn when_capacity_over() {
        let target = &mut ts::deque::normal();
        let result = test_panic(|| target.adjust_ring_start(target.capacity()));
        assert!(result.is_panic());
    }
}

#[test]
fn clone() {
    for target in ts::deques::all() {
        let result = target.clone();
        assert_eq!(result, target);
    }
}

#[test]
fn fmt() {
    let target = ts::deque::normal();
    let result = format!("{:?}", target);
    assert_eq!(result, tu::iter_dbg_text(target.iter()));
}

#[test]
fn default() {
    let result = ts::SampleDeque::default();
    assert_eq!(result.len(), 0);
    assert_eq!(result.capacity(), ts::CAPACITY);
}

#[test]
fn drop() {
    when_normal();
    when_wrapped_then_drop_items_collect_order();

    fn when_normal() {
        let target = ts::deque::normal();
        mem::drop(target);
    }

    fn when_wrapped_then_drop_items_collect_order() {
        // Arrange.
        let tracer = &DropTracer::new();
        let target = ts::deque::empty_of();
        let mut target = ch::adjust_ring_start(target, ch::RingBufAlign::Wrap);
        let vals = Vec::from_iter((0..ts::NORMAL_LEN).map(|_| tracer.trace(ts::VAL)));
        let tags = Vec::from_iter(vals.iter().map(|x| TraceVal::tag(&x)));
        target.extend(vals);

        // Act.
        mem::drop(target);

        // Assert.
        let logs = tracer.logs();
        let drop_logs = logs.iter().filter(|x| !x.is_alloc());
        let drop_tags = drop_logs.map(|x| x.tag().clone());
        assert!(drop_tags.eq(tags));
    }
}

#[test]
fn extend() {
    when_vals_normal();
    when_refs_normal();
    when_vals_capacity_over();
    when_refs_capacity_over();

    fn when_vals_normal() {
        // Arrange.
        let target = &mut ts::deque::normal();
        let master = &mut ch::vec_deque(&target);
        let len = (target.capacity() - target.len()) / 2;
        let vec = tu::random_vec::<ts::Val>(len);
        // Act.
        target.extend(vec.clone());
        master.extend(vec.clone());
        // Assert.
        assert!(target.iter().eq(master.iter()));
    }

    fn when_refs_normal() {
        // Arrange.
        let target = &mut ts::deque::normal();
        let master = &mut ch::vec_deque(&target);
        let len = (target.capacity() - target.len()) / 2;
        let vec = tu::random_vec::<ts::Val>(len);
        // Act.
        target.extend(vec.iter());
        master.extend(vec.iter());
        // Assert.
        assert!(target.iter().eq(master.iter()));
    }

    fn when_vals_capacity_over() {
        // Arrange.
        let target = &mut ts::deque::normal();
        let len = target.capacity() - target.len() + 1;
        let vec = tu::random_vec::<ts::Val>(len);
        // Act.
        let result = test_panic(|| target.extend(vec));
        // Assert.
        assert!(result.is_panic());
    }

    fn when_refs_capacity_over() {
        // Arrange.
        let target = &mut ts::deque::normal();
        let len = target.capacity() - target.len() + 1;
        let vec = tu::random_vec::<ts::Val>(len);
        // Act.
        let result = test_panic(|| target.extend(vec.iter()));
        // Assert.
        assert!(result.is_panic());
    }
}

#[test]
fn from() {
    when_from_arr_normal();
    when_from_vec_normal();
    when_from_arr_capacity_over();
    when_from_vec_capacity_over();
    when_to_vec();

    fn when_from_arr_normal() {
        // Arrange.
        const LEN: usize = ts::CAPACITY / 2;
        let vec = tu::random_vec(LEN);
        let arr = <[_; LEN]>::try_from(vec).unwrap();
        // Act.
        let result = ts::SampleDeque::from(arr.clone());
        // Assert.
        assert!(result.iter().eq(arr.iter()));
    }

    fn when_from_vec_normal() {
        // Arrange.
        let len = ts::CAPACITY / 2;
        let vec = tu::random_vec(len);
        // Act.
        let result = ts::SampleDeque::from(vec.clone());
        // Assert.
        assert!(result.iter().eq(vec.iter()));
    }

    fn when_from_arr_capacity_over() {
        // Arrange.
        const LEN: usize = ts::CAPACITY + 1;
        let vec = tu::random_vec(LEN);
        let arr = <[_; LEN]>::try_from(vec).unwrap();
        // Act.
        let result = test_panic(|| ts::SampleDeque::from(arr.clone()));
        // Assert.
        assert!(result.is_panic());
    }

    fn when_from_vec_capacity_over() {
        // Arrange.
        let len = ts::CAPACITY + 1;
        let vec = tu::random_vec(len);
        // Act.
        let result = test_panic(|| ts::SampleDeque::from(vec.clone()));
        // Assert.
        assert!(result.is_panic());
    }

    fn when_to_vec() {
        // Arrange.
        let target = ts::deque::normal();
        // Act.
        let result = Vec::from(target.clone());
        // Assert.
        assert!(result.iter().eq(target.iter()));
    }
}

#[test]
fn from_iter() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        // Arrange.
        let len = ts::CAPACITY / 2;
        let items = tu::random_vec(len);
        // Act.
        let result = ts::SampleDeque::from_iter(items.clone());
        // Assert.
        assert!(result.iter().eq(items.iter()));
    }

    fn when_capacity_over() {
        // Arrange.
        let len = ts::CAPACITY + 1;
        let items = tu::random_vec(len);
        // Act.
        let result = test_panic(|| {
            ts::SampleDeque::from_iter(items.clone());
        });
        // Assert.
        assert!(result.is_panic());
    }
}

#[test]
fn hash() {
    for [x, y] in ts::deque_pairs::all_for_cmp_normal() {
        let result_x = tu::hash(&x);
        let result_y = tu::hash(&y);
        assert!(x != y || result_x == result_y);
    }
}

#[test]
fn index() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = &ch::vec_deque(&target);
        // Act.
        let asis = test_panic(|| Index::index(target, index));
        let tobe = test_panic(|| Index::index(master, index));
        // Assert.
        assert_eqn!(asis, tobe);
    }
}

#[test]
fn index_mut() {
    let targets = ts::deques::all();
    let patterns = ts::deque_vs::each_indicies;
    for (ref mut target, index) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(&target);
        // Act.
        let asis = test_panic(|| IndexMut::index_mut(target, index));
        let tobe = test_panic(|| IndexMut::index_mut(master, index));
        // Assert.
        assert_eqn!(asis, tobe);
    }
}

#[test]
fn into_iter() {
    when_val();
    when_ref();
    when_mut();

    fn when_val() {
        for target in ts::deques::all() {
            let master = ch::vec_deque(&target);
            let asis = target.into_iter();
            let tobe = master.into_iter();
            assert!(asis.eq(tobe));
        }
    }

    fn when_ref() {
        for ref target in ts::deques::all() {
            let master = &ch::vec_deque(target);
            let asis = target.into_iter();
            let tobe = master.into_iter();
            assert!(asis.eq(tobe));
        }
    }

    fn when_mut() {
        for ref mut target in ts::deques::all() {
            let master = &mut ch::vec_deque(target);
            let asis = target.into_iter();
            let tobe = master.into_iter();
            assert!(asis.eq(tobe));
        }
    }
}

#[test]
fn cmp() {
    for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
        // Act.
        let result = target.cmp(&other);
        // Assert.
        let target_iter = target.iter();
        let other_iter = other.iter();
        assert_eq!(result, target_iter.cmp(other_iter));
    }
}

#[test]
fn eq() {
    when_normal();
    when_vec();
    when_slice_ref();
    when_slice_mut();
    when_arr();
    when_arr_ref();
    when_arr_mut();

    fn when_normal() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Act.
            let result = target.eq(&other);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }

    fn when_vec() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Arrange.
            let other_vec = other.iter().cloned().collect::<Vec<_>>();
            // Act.
            let result = target.eq(&other_vec);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other_vec.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }

    fn when_slice_ref() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Arrange.
            let other_vec = other.iter().cloned().collect::<Vec<_>>();
            let other_slice = other_vec.as_slice();
            // Act.
            let result = target.eq(&other_slice);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other_slice.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }

    fn when_slice_mut() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Arrange.
            let other_vec = &mut other.iter().cloned().collect::<Vec<_>>();
            let other_slice = other_vec.as_mut_slice();
            // Act.
            let result = target.eq(&other_slice);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other_slice.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }

    fn when_arr() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Arrange.
            let other_vec = other.iter().cloned().collect::<Vec<_>>();
            let other_arr = <[_; ts::NORMAL_LEN]>::try_from(other_vec).unwrap();
            // Act.
            let result = target.eq(&other_arr);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other_arr.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }

    fn when_arr_ref() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Arrange.
            let other_vec = other.iter().cloned().collect::<Vec<_>>();
            let other_arr = &<[_; ts::NORMAL_LEN]>::try_from(other_vec).unwrap();
            // Act.
            let result = target.eq(&other_arr);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other_arr.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }

    fn when_arr_mut() {
        for [target, other] in ts::deque_pairs::all_for_cmp_normal() {
            // Arrange.
            let other_vec = other.iter().cloned().collect::<Vec<_>>();
            let other_arr = &mut <[_; ts::NORMAL_LEN]>::try_from(other_vec).unwrap();
            // Act.
            let result = target.eq(&other_arr);
            // Assert.
            let target_iter = target.iter();
            let other_iter = other_arr.iter();
            assert_eq!(result, target_iter.eq(other_iter));
        }
    }
}

#[test]
fn partial_cmp() {
    for [target, other] in ts::deque_pairs::all_for_cmp_float() {
        // Act.
        let result = target.partial_cmp(&other);
        // Assert.
        let target_iter = target.iter();
        let other_iter = other.iter();
        assert_eq!(result, target_iter.partial_cmp(other_iter));
    }
}

#[test]
fn read() {
    // Note: The test approach comparing against the master cannot be used.
    // This is because the amount of data read by `VecDeque::read` depends
    // on the buffer split position, which allows for some flexibility.
    // As a result, comparing the results becomes unreliable.
    let targets = ts::deques::all_of();
    let patterns = ts::deque_vs::each_indicies;
    for (ref mut target, buf_len) in targets.flat_map(patterns) {
        // Arrange.
        let buf = &mut tu::buf(buf_len);
        let to_read = Vec::from_iter(target.as_slices().0.iter().cloned());
        // Act.
        let result = target.read(buf);
        // Assert success.
        assert!(result.as_ref().is_ok());
        // Assert reading contents.
        let read_cnt = result.unwrap();
        assert!(read_cnt == to_read.len().min(buf_len));
        assert_eq!(buf[0..read_cnt], to_read[0..read_cnt]);
    }
}

#[test]
fn write() {
    when_normal();
    when_capacity_over();

    fn when_normal() {
        let targets = ts::deques::all_of();
        let patterns = ts::deque_vs::each_indicies;
        let cond = |x: &(ArrDeque<u8, 30>, usize)| x.0.len() + x.1 <= x.0.capacity();
        for (ref mut target, buf_len) in targets.flat_map(patterns).filter(cond) {
            // Arrange.
            let master = &mut ch::vec_deque(&target);
            let buf = &tu::random_vec(buf_len);
            // Act.
            let asis = test_panic(|| target.write(buf));
            let tobe = test_panic(|| master.write(buf));
            // Assert.
            assert_eq!(asis.is_cool(), tobe.is_cool());
            assert!(target.iter().eq(master.iter()));
        }
    }

    fn when_capacity_over() {
        let targets = ts::deques::all_of();
        let patterns = ts::deque_vs::each_indicies;
        let cond = |x: &(ArrDeque<u8, 30>, usize)| x.0.len() + x.1 > x.0.capacity();
        for (ref mut target, buf_len) in targets.flat_map(patterns).filter(cond) {
            // Arrange.
            let memo = target.clone();
            let buf = &tu::random_vec(buf_len);
            // Act.
            let result = test_panic(|| target.write(buf));
            // Assert.
            assert!(result.is_panic());
            assert!(target.iter().eq(memo.iter()))
        }
    }
}

#[test]
fn flush() {
    let target = &mut ts::deque::type_byte();
    let result = target.flush();
    assert!(result.is_ok());
}

#[test]
fn fill_buf() {
    // Note: The test approach comparing against the master cannot be used.
    // This is because the amount of data read by `VecDeque::read` depends
    // on the buffer split position, which allows for some flexibility.
    // As a result, comparing the results becomes unreliable.
    for ref mut target in ts::deques::all_of() {
        // Arrange.
        let to_read = Vec::from_iter(target.as_slices().0.iter().cloned());
        // Act.
        let result = target.fill_buf();
        // Assert.
        assert!(result.as_ref().is_ok());
        assert_eq!(result.unwrap(), to_read);
    }
}

#[test]
fn consume() {
    let targets = ts::deques::all_of();
    let patterns = ts::deque_vs::each_indicies_more;
    for (ref mut target, amt) in targets.flat_map(patterns) {
        // Arrange.
        let master = &mut ch::vec_deque(&target);
        // Act.
        let asis = test_panic(|| target.consume(amt));
        let tobe = test_panic(|| master.consume(amt));
        // Assert.
        assert_eqn!(asis, tobe);
        assert!(target.iter().eq(master.iter()));
    }
}
