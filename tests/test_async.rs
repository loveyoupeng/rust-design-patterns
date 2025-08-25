use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rust_design_patterns::async_impl::SleepFuture;

async fn generator(a: i32, b: i32) -> i32 {
    a + b
}
fn future_method(duration: Duration) -> impl Future<Output = u128> {
    SleepFuture::new(duration)
}

#[tokio::test]
async fn test_generator() {
    let value = generator(1, 2).await;
    let duration = Duration::from_secs(1);
    let now = SystemTime::now() + duration;
    let result = now
        .duration_since(UNIX_EPOCH)
        .expect("should work")
        .as_millis();
    assert!(result <= future_method(duration).await);
    assert_eq!(3, value);
}
