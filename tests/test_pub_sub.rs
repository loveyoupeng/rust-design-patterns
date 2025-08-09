use rust_design_patterns::pub_sub::create_buffer;
use std::thread;

#[test]
fn test_pub_sub() {
    let (publisher, subscriber) = create_buffer::<i32>(1024);
    let size = 1_000;
    thread::scope(|scope| {
        scope.spawn(|| {
            for i in 1..size {
                while !publisher.try_offer(i) {}
            }
        });
        scope.spawn(|| {
            for index in 1..size {
                while match subscriber.try_poll() {
                    None => true,
                    Some(value) => {
                        assert_eq!(index, value);
                        false
                    }
                } {}
            }
        });
    });
}
