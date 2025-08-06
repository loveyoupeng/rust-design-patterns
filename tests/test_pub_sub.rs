use rust_design_patterns::pub_sub::create_buffer;
use std::thread;

#[test]
fn test_pub_sub() {
    let (publisher, subscriber) = create_buffer::<i32>(1024);
    thread::scope(|scope| {
        scope.spawn(|| {
            for i in 1..10 {
                publisher.publish(i);
            }
        });
        scope.spawn(|| {
            for _ in 1..10 {
                assert_eq!(None, subscriber.try_poll());
            }
        });
    });
}
