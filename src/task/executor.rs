//! TODO:
//! - Add `Spawner` struct to always be able to spawn new tasks.
//! - Add threading and work stealing
use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;
use spin::RwLock;

use super::{Task, TaskId};

const MAX_AMOUNT_OF_QUEUED_TASKS: usize = 128;

pub struct Executor {
    /// Fast search and continuation of a task
    tasks: Arc<RwLock<BTreeMap<TaskId, Task>>>,
    /// Shared with wakers, which push their task onto it
    task_queue: Arc<ArrayQueue<TaskId>>,
    /// Allows reuse of wakers and ?
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        Executor {
            tasks: Arc::new(RwLock::new(BTreeMap::new())),
            task_queue: Arc::new(ArrayQueue::new(MAX_AMOUNT_OF_QUEUED_TASKS)),
            waker_cache: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn get_spawner(&self) -> Spawner {
        Spawner {
            tasks: self.tasks.clone(),
            task_queue: self.task_queue.clone(),
        }
    }

    /// Runs until all queued tasks finish.
    pub fn run(&mut self) {
        while !self.tasks.read().is_empty() {
            self.run_ready_task();
            self.sleep_if_idle();
        }
    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};
        interrupts::disable();
        // Check if a interrupt queues a new task inbetween
        if self.task_queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    fn run_ready_task(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(id) = task_queue.pop() {
            let mut tasks = tasks.write();
            let task = match tasks.get_mut(&id) {
                Some(task) => task,
                None => continue, // Task finished already
            };
            let waker = waker_cache
                .entry(id)
                .or_insert_with(|| TaskWaker::new(id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    tasks.remove(&id);
                    waker_cache.remove(&id);
                }
                Poll::Pending => {}
            }
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    tasks: Arc<RwLock<BTreeMap<TaskId, Task>>>,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

// Requires tasks
impl Spawner {
    /// # Panics
    /// Panics if the amount of queued tasks exceeds `MAX_AMOUNT_OF_QUEUED_TASKS`
    pub fn spawn(&mut self, task: Task) {
        let id = task.id;
        assert!(
            self.tasks.write().insert(id, task).is_none(),
            "Task with {:?} already exists",
            id
        );
        assert!(
            self.task_queue.push(id).is_ok(),
            "Max amount of queued tasks reached: {}",
            MAX_AMOUNT_OF_QUEUED_TASKS
        );
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    #[allow(clippy::new_ret_no_self)]
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_id,
            task_queue,
        }))
    }

    /// # Panics
    /// Panics if the amount of queued tasks exceeds `MAX_AMOUNT_OF_QUEUED_TASKS`
    fn wake_task(&self) {
        assert!(
            self.task_queue.push(self.task_id).is_ok(),
            "Max amount of queued tasks reached: {}",
            MAX_AMOUNT_OF_QUEUED_TASKS
        );
    }
}

impl Wake for TaskWaker {
    #[inline]
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    #[inline]
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
