use std::marker::PhantomData;

pub struct Publiser<T: Clone> {
    _phantom: PhantomData<T>,
}
pub struct Subscriber<T: Clone> {
    _phantom: PhantomData<T>,
}

impl<T: Clone> Publiser<T> {
    pub fn publish(&self, _value: T) {}
}

impl<T: Clone> Subscriber<T> {
    pub fn try_poll(&self) -> Option<T> {
        None
    }
}

unsafe impl<T: Clone> Send for Publiser<T> {}
unsafe impl<T: Clone> Send for Subscriber<T> {}

pub fn create_buffer<T: Clone>(_: usize) -> (Publiser<T>, Subscriber<T>) {
    (
        Publiser {
            _phantom: PhantomData,
        },
        Subscriber {
            _phantom: PhantomData,
        },
    )
}
