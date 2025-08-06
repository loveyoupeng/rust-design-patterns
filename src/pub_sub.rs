use std::{marker::PhantomData, sync::Arc};

pub struct Buffer<T: Send + Copy> {
    values: Vec<Option<T>>,
}

unsafe impl<T: Send + Copy> Sync for Buffer<T> {}

impl<T: Send + Copy> Default for Buffer<T> {
    fn default() -> Self {
        Self { values: vec![] }
    }
}

pub struct Publiser<T: Send + Copy> {
    phantom: PhantomData<T>,
}
pub struct Subscriber<T: Send + Copy> {
    phantom: PhantomData<T>,
}

impl<T: Send + Copy> Publiser<T> {
    pub fn publish(&self, _value: T) {}
}

impl<T: Send + Copy> Subscriber<T> {
    pub fn try_poll(&self) -> Option<T> {
        None
    }
}

unsafe impl<T: Send + Copy> Send for Publiser<T> {}
unsafe impl<T: Send + Copy> Send for Subscriber<T> {}

pub fn create_buffer<T: Send + Copy>(_size: usize) -> (Publiser<T>, Subscriber<T>) {
    (
        Publiser {
            phantom: PhantomData,
        },
        Subscriber {
            phantom: PhantomData,
        },
    )
}
