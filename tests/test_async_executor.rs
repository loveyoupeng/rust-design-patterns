use rust_design_patterns::async_executor::new_executor_and_spawner;

async fn do_nothing() -> () {
    print!("finished");
}

#[test]
fn test_executor() {
    print!("test executor");
    let (executor, spawner) = new_executor_and_spawner();
    spawner.spawn(do_nothing());
    drop(spawner);
    executor.run();
}
