trait Builder<T, R>
where
    T: Sized,
{
    fn with_name(self, name: String) -> Self;
    fn with_init_value(self, value: T) -> Self;
}

struct Node<T>
where
    T: Sized,
{
    name: String,
    init_value: Option<T>,
}

struct NodeBuilder<T>
where
    T: Sized,
{
    value: Node<T>,
}

impl<T> NodeBuilder<T>
where
    T: Sized,
{
    fn new() -> Self {
        NodeBuilder {
            value: Node {
                name: String::new(),
                init_value: None,
            },
        }
    }
    fn build(self) -> Node<T> {
        self.value
    }
}

impl<T> Builder<T, Node<T>> for NodeBuilder<T>
where
    T: Sized,
{
    fn with_name(mut self, name: String) -> Self {
        self.value.name = name;
        self
    }
    fn with_init_value(mut self, value: T) -> Self {
        self.value.init_value = Some(value);
        self
    }
}

#[test]
fn test_builder() {
    let node = NodeBuilder::new()
        .with_name(String::from("abc"))
        .with_init_value(10)
        .build();
    assert_eq!(Some(10), node.init_value);
    assert_eq!(String::from("abc"), node.name);
}
