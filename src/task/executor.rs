use alloc::{collections::BTreeMap, sync::Arc, task::Wake};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

pub use super::simple_executor::SimpleExecutor;
use super::{Task, TaskId};

const MAX_AMOUNT_OF_QUEUED_TASKS: usize = 128;

pub struct Executor {
    /// Fast search and continuation of a task
    tasks: BTreeMap<TaskId, Task>,
    /// Shared with wakers, which push their task onto it
    task_queue: Arc<ArrayQueue<TaskId>>,
    /// Allows reuse of wakers and ?
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(MAX_AMOUNT_OF_QUEUED_TASKS)),
            waker_cache: BTreeMap::new(),
        }
    }

    /// FIXME: Use custom, copy-able Spawner struct
    pub fn spawn(&mut self, task: Task) {
        let id = task.id;
        if self.tasks.insert(id, task).is_some() {
            panic!("Task with {:?} already exists", id);
        }
        if self.task_queue.push(id).is_err() {
            panic!(
                "Max amount of queued tasks reached: {}",
                MAX_AMOUNT_OF_QUEUED_TASKS,
            );
        }
    }

    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            self.run_ready_task();
            self.sleep_if_idle();
        }
    }

    fn sleep_if_idle(&self) {
        // Check if a interrupt queues a new task inbetween
        if self.task_queue.is_empty() {
            crate::hal::hlt();
        }
    }

    fn run_ready_task(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(id) = task_queue.pop() {
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

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        if self.task_queue.push(self.task_id).is_err() {
            panic!(
                "Max amount of queued tasks reached: {}",
                MAX_AMOUNT_OF_QUEUED_TASKS,
            );
        }
    }
}

impl Wake for TaskWaker {
    #[inline]
    fn wake(self: Arc<Self>) {
        self.wake_task()
    }

    #[inline]
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}
