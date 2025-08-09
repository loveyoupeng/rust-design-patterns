use rust_design_patterns::pub_sub::create_buffer;
use std::thread;

#[test]
fn test_pub_sub() {
    let (publisher, subscriber) = create_buffer::<i32>(1024);
    thread::scope(|scope| {
        scope.spawn(|| {
            for i in 1..10 {
                while !publisher.try_offer(i) {}
            }
        });
        scope.spawn(|| {
            for index in 1..10 {
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
