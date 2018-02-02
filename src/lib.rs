#[macro_use]
extern crate log;
extern crate futures;

use std::fmt;

use futures::{Future, Poll, Stream};

/// A wrapper around a `Future` or `Stream` that logs calls to `poll`
/// and their results.
#[derive(Debug, Clone)]
pub struct Trace<T> {
    name: Option<&'static str>,
    inner: T,
}
impl<T> Trace<T> {

    /// Add a name for the underlying value, to be used
    /// instead of calling fmt::Debug.
    pub fn named(self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    fn get_name(&self) -> &str {
        self.name.unwrap_or_else(|| {
            &format!("{:?}", self.inner)
        })
    }

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
        trace!("{:?}.poll()", self.get_name());
        let poll = self.inner.poll();
        trace!("{:?}.poll() -> {:?};", self.get_name(), poll);
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
        trace!("{:?}.poll()", self.get_name());
        let poll = self.inner.poll();
        trace!("{:?}.poll() -> {:?};", self.get_name(), poll);
        poll
    }

}

impl<T> From<T> for Trace<T> {

    fn from(inner: T) -> Self {
        Trace {
            inner,
            name: None,
        }
    }

}
