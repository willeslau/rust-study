use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

pub struct Sleep {
    state: Arc<Mutex<Shared>>,
}

impl Sleep {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(Shared {
            completed: false,
            waker: None,
        }));

        let s = state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = s.lock().unwrap();
            state.completed = true;
            if let Some(w) = state.waker.take() {
                w.wake()
            }
        });

        Self { state }
    }
}

struct Shared {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            return Poll::Ready(());
        }

        // sleep not ready yet, continue polling
        state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}
