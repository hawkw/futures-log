#[macro_use]
extern crate log;
extern crate futures;

use std::default::Default;
use std::fmt;

use futures::{Future, Poll, Stream};

/// A wrapper around a `Future` or `Stream` that logs calls to `poll`
/// and their results.
#[derive(Clone)]
pub struct Trace<T> {
    name: Option<&'static str>,
    level: log::Level,
    inner: T,
}

impl<T> Trace<T> {

    /// Add a name for the underlying value, to be used
    /// instead of calling fmt::Debug.
    pub fn named(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    /// Change the log level.
    pub fn at_level(mut self, level: log::Level) -> Self {
        self.level = level;
        self
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
        log!(self.level, "{:?}.poll();", self);
        let poll = self.inner.poll();
        log!(self.level, "{:?}.poll() -> {:?};", self, poll);
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
        log!(self.level, "{:?}.poll();", self);
        let poll = self.inner.poll();
        log!(self.level, "{:?}.poll() -> {:?};", self, poll);
        poll
    }

}

impl<T> From<T> for Trace<T> {

    fn from(inner: T) -> Self {
        Trace {
            name: None,
            level: log::Level::Trace,
            inner,
        }
    }

}


impl<T> fmt::Debug for Trace<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref name) = self.name {
            write!(f, "{}", name)
        } else {
            write!(f, "{:?}", self.inner)
        }

    }
}
