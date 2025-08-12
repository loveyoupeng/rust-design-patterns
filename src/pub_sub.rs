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

    /// Attempts to add a `value` to the queue.
    ///
    /// This method checks if there is available capacity in the queue. If the queue
    /// is full, it returns `false` immediately without adding the value.
    /// If there is capacity, the value is written to the next available slot,
    /// and the tail pointer is advanced.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of type `T` to be added to the queue.
    ///
    /// # Returns
    ///
    /// `true` if the value was successfully added to the queue, `false` if the
    /// queue is currently full.
    ///
    /// # Safety
    ///
    /// This method uses `unsafe` to access the `values` array, assuming that `self.values`
    /// points to a valid, mutable slice of `MaybeUninit<ManuallyDrop<T>>` and that
    /// `index` is within bounds `[0, self.capacity)`. The `ManuallyDrop` ensures that
    /// the value is not dropped automatically when it goes out of scope, leaving
    /// drop responsibility to the consumer.
    ///
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
