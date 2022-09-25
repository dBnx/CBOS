use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use super::Task;
use alloc::collections::VecDeque;

pub struct SimpleExecutor {
    task_queue: VecDeque<Task>,
}

impl SimpleExecutor {
    pub fn new() -> SimpleExecutor {
        SimpleExecutor {
            task_queue: VecDeque::with_capacity(8),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }

    pub fn run(&mut self) {
        let waker = dummy_waker();
        while let Some(mut task) = self.task_queue.pop_front() {
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {}
                Poll::Pending => self.task_queue.push_back(task),
            }
        }
    }
}

// TODO: pub struct Spawner {}

fn dummy_raw_waker() -> RawWaker {
    let vtable = {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            dummy_raw_waker()
        }
        // Vtable entries: clone, wake, wake_by_ref, drop
        &RawWakerVTable::new(clone, no_op, no_op, no_op)
    };
    RawWaker::new(0 as *const (), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}
