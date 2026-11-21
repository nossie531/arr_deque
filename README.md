# arr_deque

Deque implemented by array.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

Core item of this crate is [`ArrDeque`].\
This item is similar to [`VecDeque`] in [`std`].\
But this type uses array instead of vector for internal ring buffer.

## Other options

There are similar crates with greater track records.

Followings are some of them.

📦 [`arraydeque`](https://crates.io/crates/arraydeque)\
📦 [`array-deque`](https://crates.io/crates/array-deque)

I recommend these crates over my crate if there are no special reasons.

## Highlights

This crate focuses on the following.

- Similarity to [`VecDeque`].
- No UB (Checked by MIRI).

## History

See [CHANGELOG](CHANGELOG.md).

<!-- links -->
[`std`]: https://doc.rust-lang.org/std/
[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
[`ArrDeque`]: https://docs.rs/arr_deque/0.5.0/arr_deque/struct.ArrDeque.html
