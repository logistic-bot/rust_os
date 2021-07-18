use super::Task;
use alloc::collections::VecDeque;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Very simple executor, executing each task sequentially until all are done.
pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    /// Create new Executor with no queued tasks
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    /// Add given task to the queue
    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task)
    }

    /// Run all tasks in the queue, until everything is done
    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {} // Task done
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

impl Default for SimpleExecutor {
    fn default() -> Self {
        Self::new()
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, noop, noop, noop);
    #[allow(clippy::zero_ptr)] // due to no_std, we cant use std::ptr::null
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
