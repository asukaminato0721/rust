//@ run-pass
//@ edition: 2024

#![feature(async_iterator, gen_blocks)]

//! Regression test for <https://github.com/rust-lang/rust/issues/124751>.
//! `async gen` closures should lower and type-check just like other
//! coroutine closures without causing an ICE.

use std::async_iter::AsyncIterator;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

fn main() {
    let make_iter = async gen || -> i32 {
        yield 1;
        yield 2;
    };

    let mut iter = pin!(make_iter());
    let ctx = &mut Context::from_waker(Waker::noop());

    assert!(matches!(iter.as_mut().poll_next(ctx), Poll::Ready(Some(1))));
    assert!(matches!(iter.as_mut().poll_next(ctx), Poll::Ready(Some(2))));
    assert!(matches!(iter.as_mut().poll_next(ctx), Poll::Ready(None)));
}
