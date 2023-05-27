use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Future;

#[derive(Debug)]
pub struct Futures<T> {
    task: T,
    completed: bool,
}

impl<T> Futures<T> {
    pub fn new(task: T) -> Self {
        Futures {
            task,
            completed: false,
        }
    }
}

impl<T: Future<Output = ()>> Future for Futures<T> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready(())
        } else {
            let task = Pin::new(&mut self.task);
            let poll_result = task.poll(cx);
            if poll_result.is_ready() {
                self.completed = true;
            }
            poll_result
        }
    }
}
