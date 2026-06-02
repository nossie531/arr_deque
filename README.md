# arr_deque

Deque implemented by array.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

Core item of this crate is [`ArrDeque`].
This type is similar to [`VecDeque`].
But this type uses array instead of vector for internal ring buffer.

## Other options

There are similar crates with greater track records.

Followings are some of them.

📦 [`arraydeque`](https://crates.io/crates/arraydeque)\
📦 [`array-deque`](https://crates.io/crates/array-deque)

I recommend these over my crate if there is special reason.

## Highlights

This crate focuses on the following.

- Similarity to [`VecDeque`].
- No UB (Checked by MIRI).

## History

See [CHANGELOG](CHANGELOG.md).

<!-- links -->
[`ArrDeque`]: https://docs.rs/arr_deque/0.1.1/arr_deque/struct.ArrDeque.html
[`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
