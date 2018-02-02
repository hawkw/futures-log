futures-trace
=============

A very simple wrapper around  [`futures-rs`] `Future`s and `Stream`s that log
all calls to `poll` and their results, to aid in debugging.

[`futures-rs`]: https://github.com/rust-lang-nursery/futures-rs