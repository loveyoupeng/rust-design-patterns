use rust_design_patterns::pub_sub::create_buffer;
use std::{hint::spin_loop, thread};

struct TestData {
    value: i32,
    name: String,
}

#[test]
fn test_pub_sub() {
    let (publisher, subscriber) = create_buffer::<TestData>(1024);
    let size = 10_000_000;
    thread::scope(|scope| {
        scope.spawn(|| {
            for i in 1..size {
                while !publisher.try_offer(TestData {
                    value: i,
                    name: format!("name-{i}"),
                }) {
                    spin_loop();
                }
            }
        });
        scope.spawn(|| {
            for index in 1..size {
                while match subscriber.try_poll() {
                    None => true,
                    Some(value) => {
                        assert_eq!(index, value.value);
                        let name = format!("name-{index}");
                        assert_eq!(name, value.name);
                        false
                    }
                } {
                    spin_loop();
                }
            }
        });
    });
}
