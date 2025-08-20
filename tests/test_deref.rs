use std::ops::Deref;
struct DataHolder<T>(T);

impl<T> DataHolder<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for DataHolder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Eq)]
struct Data {
    value: i32,
}

impl Data {
    /// Creates a new `Data` instance with an initial value.
    ///
    /// # Arguments
    ///
    /// * `init_value` - The initial integer value for the `Data` struct.
    ///
    /// # Returns
    ///
    /// A new `Data` instance.
    fn new(init_value: i32) -> Self {
        Self { value: init_value }
    }

    /// Adds one to the internal `value` and returns the result.
    ///
    /// This method does not modify the `Data` instance itself.
    ///
    /// # Returns
    ///
    /// The `value` incremented by one.
    fn add_one(&self) -> i32 {
        self.value + 1
    }
}

#[test]
fn test_deref() {
    let dh = DataHolder::new(Data::new(10));
    assert_eq!(11, dh.add_one());
    assert!(Data::new(10) == *dh);
}
