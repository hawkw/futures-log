#[macro_use]
extern crate log;
extern crate futures;

use std::fmt;

use futures::{Future, Poll, Stream};

/// A wrapper around a `Future` or `Stream` that logs calls to `poll`
/// and their results.
#[derive(Debug, Clone)]
pub struct Trace<T> {
    inner: T,
}

impl<F> Future for Trace<F>
where
    F: Future,
    F::Item: fmt::Debug,
    F::Error: fmt::Debug,
    F: fmt::Debug,
{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        trace!("{:?}.poll()", self.inner);
        let poll = self.inner.poll();
        trace!("{:?}.poll() -> {:?};", self.inner, poll);
        poll
    }

}

impl<S> Stream for Trace<S>
where
    S: Stream,
    S::Item: fmt::Debug,
    S::Error: fmt::Debug,
    S: fmt::Debug,
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        trace!("{:?}.poll()", self.inner);
        let poll = self.inner.poll();
        trace!("{:?}.poll() -> {:?};", self.inner, poll);
        poll
    }

}

impl<T> From<T> for Trace<T> {

    fn from(inner: T) -> Self {
        Trace { inner }
    }

}
