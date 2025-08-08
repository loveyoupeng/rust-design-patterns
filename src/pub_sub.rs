use std::{
    cell::RefCell,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

pub struct Buffer<T: Send + Copy> {
    values: RefCell<Vec<T>>,
    tail: AtomicU64,
    head: AtomicU64,
    capacity: u64,
    mask: u64,
}

impl<T: Send + Copy> Buffer<T> {
    fn new(size: usize, init_value: T) -> Self {
        Self {
            values: RefCell::new(vec![init_value; size]),
            tail: AtomicU64::new(0),
            head: AtomicU64::new(0),
            capacity: size as u64,
            mask: (size - 1) as u64,
        }
    }

    fn try_offer(&self, value: T) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);
        if tail - head > self.capacity {
            return false;
        }
        let index = (tail & self.mask) as usize;
        let mut values = self.values.borrow_mut();
        values[index] = value;
        self.tail.store(tail + 1, Ordering::Release);
        true
    }

    fn try_poll(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if tail == head {
            return None;
        }
        let index = (head & self.mask) as usize;
        let values = self.values.borrow();
        let result = values[index];
        self.head.store(head + 1, Ordering::Release);
        Some(result)
    }
}

unsafe impl<T: Send + Copy> Sync for Buffer<T> {}

pub struct Publiser<T: Send + Copy> {
    buffer: Arc<Buffer<T>>,
}
pub struct Subscriber<T: Send + Copy> {
    buffer: Arc<Buffer<T>>,
}

impl<T: Send + Copy> Publiser<T> {
    pub fn try_offer(&self, value: T) -> bool {
        self.buffer.try_offer(value)
    }
}

impl<T: Send + Copy> Subscriber<T> {
    pub fn try_poll(&self) -> Option<T> {
        self.buffer.try_poll()
    }
}

unsafe impl<T: Send + Copy> Send for Publiser<T> {}
unsafe impl<T: Send + Copy> Send for Subscriber<T> {}

pub fn create_buffer<T: Send + Copy>(size: usize, init_value: T) -> (Publiser<T>, Subscriber<T>) {
    let buffer = Arc::new(Buffer::new(size, init_value));
    (
        Publiser {
            buffer: buffer.clone(),
        },
        Subscriber { buffer },
    )
}
