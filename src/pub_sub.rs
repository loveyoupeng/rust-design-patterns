use std::{
    cell::UnsafeCell,
    mem::{ManuallyDrop, MaybeUninit},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

pub struct Buffer<T> {
    values: UnsafeCell<Vec<MaybeUninit<ManuallyDrop<T>>>>,
    tail: AtomicU64,
    head: AtomicU64,
    capacity: u64,
    mask: u64,
}

impl<T> Buffer<T> {
    fn new(size: usize) -> Self {
        let mut vec: Vec<MaybeUninit<ManuallyDrop<T>>> = Vec::with_capacity(size);
        for _ in 0..size {
            vec.push(MaybeUninit::uninit());
        }

        Self {
            values: UnsafeCell::new(vec),
            tail: AtomicU64::new(0),
            head: AtomicU64::new(0),
            capacity: size as u64,
            mask: (size - 1) as u64,
        }
    }

    fn try_offer(&self, value: T) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Relaxed);
        if tail - head >= self.capacity {
            return false;
        }
        let index = (tail & self.mask) as usize;
        let values = unsafe { &mut *self.values.get() };
        values[index].write(ManuallyDrop::new(value));
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
        let values = unsafe { &*self.values.get() };
        let value = unsafe { values[index].assume_init_read() };
        let result = ManuallyDrop::into_inner(value);
        self.head.store(head + 1, Ordering::Release);
        Some(result)
    }
}

unsafe impl<T> Sync for Buffer<T> {}

pub struct Publiser<T> {
    buffer: Arc<Buffer<T>>,
}
pub struct Subscriber<T> {
    buffer: Arc<Buffer<T>>,
}

impl<T> Publiser<T> {
    pub fn try_offer(&self, value: T) -> bool {
        self.buffer.try_offer(value)
    }
}

impl<T> Subscriber<T> {
    pub fn try_poll(&self) -> Option<T> {
        self.buffer.try_poll()
    }
}

unsafe impl<T> Send for Publiser<T> {}
unsafe impl<T> Send for Subscriber<T> {}

pub fn create_buffer<T>(size: usize) -> (Publiser<T>, Subscriber<T>) {
    let buffer = Arc::new(Buffer::new(size));
    (
        Publiser {
            buffer: buffer.clone(),
        },
        Subscriber { buffer },
    )
}
