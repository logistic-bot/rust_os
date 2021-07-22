use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, Ordering};
use core::task::{Context, Poll};
use core::{future::Future, pin::Pin};

/// Contains a very simple executor, executing each task sequentially until all are done
pub mod simple_executor;

/// Simple single-threaded executor with waker support
pub mod executor;

/// Async keyboard driver
pub mod keyboard;

/// A task that contains a future returning ()
pub struct Task {
    id: TaskID,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Create a new task from a Future.
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        // The 'static lifetime is required here because the returned Task can live for
        // an arbitrary time, so the future needs to be valid for that time too
        Self {
            id: TaskID::new(),
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskID(u64);

impl TaskID {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskID(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
