use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    sync::atomic::{AtomicU64, Ordering},
};

pub struct Buffer<T: Send> {
    values: Vec<UnsafeCell<Option<T>>>,
    tail: AtomicU64,
    head: AtomicU64,
    capacity: u64,
}

impl<T: Send> Buffer<T> {
    fn new(size: usize) -> Self {
        let mut _values = Vec::with_capacity(size);
        for _ in 0..size {
            _values.push(UnsafeCell::new(None))
        }

        Self {
            values: _values,
            tail: AtomicU64::new(0),
            head: AtomicU64::new(0),
            capacity: (size - 1) as u64,
        }
    }

    fn try_offer(&self, value: T) -> bool {
        let head = self.head.load(std::sync::atomic::Ordering::Acquire);
        let index = (head & self.capacity) as usize;
        unsafe {
            let cell = self.values.get_unchecked(index);
            let ptr = cell.get();
            ptr.write(Some(value));
        }
        self.head.store(head + 1, Ordering::Release);
        true
    }
}

unsafe impl<T: Send> Sync for Buffer<T> {}

impl<T: Send> Default for Buffer<T> {
    fn default() -> Self {
        Buffer::new(1024)
    }
}

pub struct Publiser<T: Send> {
    _buffer: Buffer<T>,
}
pub struct Subscriber<T: Send> {
    phantom: PhantomData<T>,
}

impl<T: Send> Publiser<T> {
    pub fn try_offer(&self, value: T) -> bool {
        self._buffer.try_offer(value)
    }
}

impl<T: Send> Subscriber<T> {
    pub fn try_poll(&self) -> Option<T> {
        None
    }
}

unsafe impl<T: Send> Send for Publiser<T> {}
unsafe impl<T: Send> Send for Subscriber<T> {}

pub fn create_buffer<T: Send>(size: usize) -> (Publiser<T>, Subscriber<T>) {
    let buffer: Buffer<T> = Buffer::new(size);
    (
        Publiser { _buffer: buffer },
        Subscriber {
            phantom: PhantomData,
        },
    )
}
