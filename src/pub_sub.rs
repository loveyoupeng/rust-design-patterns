use std::marker::PhantomData;

pub struct Buffer<T: Send, const N: usize> {
    values: [Option<T>; N],
}

pub struct Publiser<T: Send> {
    _phantom: PhantomData<T>,
}
pub struct Subscriber<T: Send> {
    _phantom: PhantomData<T>,
}

impl<T: Send> Publiser<T> {
    pub fn publish(&self, _value: T) {}
}

impl<T: Send> Subscriber<T> {
    pub fn try_poll(&self) -> Option<T> {
        None
    }
}

unsafe impl<T: Send> Send for Publiser<T> {}
unsafe impl<T: Send> Send for Subscriber<T> {}

pub fn create_buffer<T: Send, const N: usize>() -> (Publiser<T>, Subscriber<T>) {
    (
        Publiser {
            _phantom: PhantomData,
        },
        Subscriber {
            _phantom: PhantomData,
        },
    )
}
