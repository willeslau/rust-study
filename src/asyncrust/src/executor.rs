use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use futures::FutureExt;
use std::cell::SyncUnsafeCell;
use std::future::Future;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::task::Context;

// The overall flow for future is as follows:
//   spawner submits the FUTURE to queue
//   executor takes from the queue, poll the FUTURE,
//       while the actual work of the FUTURE is being processed by third party
//   future is re-queued by the waker if necessary
/// Pulls from the ready queue and executes the tasks
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        loop {
            if let Ok(task) = self.ready_queue.recv() {
                let f_raw = unsafe { &mut *task.future.get() };
                let future = f_raw.take();
                if let Some(mut f) = future {
                    // Create a `LocalWaker` from the task itself
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&waker);
                    if f.as_mut().poll(context).is_pending() {
                        *f_raw = Some(f);
                    }
                }
            }
        }
    }
}

/// Spawns a future
#[derive(Clone)]
pub struct Spawner {
    task_sender: Sender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Task {
            future: SyncUnsafeCell::new(Some(future.boxed())),
            task_sender: self.task_sender.clone(),
        };
        self.task_sender
            .send(Arc::new(task))
            .expect("too many tasks queued");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    let (tx, rx) = channel();
    let executor = Executor { ready_queue: rx };
    let spawner = Spawner { task_sender: tx };
    (executor, spawner)
}

struct Task {
    // Actually this SyncUnsafeCell might not be needed. We can force Task to be Sync
    // since the only update is in executor thread.
    future: SyncUnsafeCell<Option<BoxFuture<'static, ()>>>,
    // We need this so that waker can requeue the task.
    task_sender: Sender<Arc<Task>>,
}

unsafe impl Sync for Task {}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let task = arc_self.clone();
        arc_self
            .task_sender
            .send(task)
            .expect("too many tasks queued");
    }
}
