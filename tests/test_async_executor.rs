use std::time::Duration;

use rust_design_patterns::{async_executor::new_executor_and_spawner, async_impl::SleepFuture};

async fn do_nothing() -> () {
    println!("before sleep");
    let value = SleepFuture::new(Duration::from_secs(1)).await;
    println!("finished with {value}");
}

#[test]
fn test_executor() {
    println!("test executor");
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(do_nothing());
    drop(spawner);
    executor.run();
}
