fn execute<F>(func: F) -> i32
where
    F: Fn(i32) -> i32,
{
    func(0)
}
#[test]
fn test_strategy() {
    assert_eq!(0, execute(|i| { i }));
}
