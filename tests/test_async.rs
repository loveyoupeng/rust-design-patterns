async fn generator(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::test]
async fn test_generator() {
    let value = generator(1, 2).await;
    assert_eq!(3, value);
}
