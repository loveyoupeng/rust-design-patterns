use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub struct State {
    current_state: u8,
    waker: Option<Waker>,
}

impl State {
    fn new() -> Self {
        Self {
            current_state: 0,
            waker: None,
        }
    }
}

pub struct SleepFuture {
    expiration: SystemTime,
    result: u128,
    state: Arc<Mutex<State>>,
}

impl SleepFuture {
    pub fn new(duration: Duration) -> Self {
        let exp = SystemTime::now() + duration;
        Self {
            expiration: exp,
            result: exp
                .duration_since(UNIX_EPOCH)
                .expect("should work")
                .as_millis(),
            state: Arc::new(Mutex::new(State::new())),
        }
    }
}

impl Future for SleepFuture {
    type Output = u128;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.current_state == 2 {
            Poll::Ready(self.result)
        } else if state.current_state == 0 {
            state.waker = Some(cx.waker().clone());
            let lock = self.state.clone();
            let expiration = self.expiration;
            thread::spawn(move || {
                loop {
                    thread::sleep(Duration::from_millis(10));
                    if SystemTime::now() >= expiration {
                        let mut state = lock.lock().unwrap();
                        state.current_state = 2_u8;
                        state.waker.take().unwrap().wake();
                    }
                }
            });
            state.current_state = 1;
            Poll::Pending
        } else {
            Poll::Pending
        }
    }
}
