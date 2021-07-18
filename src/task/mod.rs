use alloc::boxed::Box;
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

/// Contains a very simple executor, executing each task sequentially until all are done
pub mod simple_executor;

/// A task that contains a future returning ()
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a new task from a Future.
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        // The 'static lifetime is required here because the returned Task can live for
        // an arbitrary time, so the future needs to be valid for that time too
        Self {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
