use std::sync::mpsc::channel;

use rust_design_patterns::i32_getter;

#[test]
fn test_fileds_macro() {
    let bytes: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
    let value: i32 = i32_getter!(bytes);
    assert_eq!(3, value);
}

#[test]
fn test_channel() {
    let (tx, rx) = channel::<i32>();
    match tx.send(10) {
        Ok(_) => {}
        Err(_) => unreachable!(),
    }
    match rx.recv() {
        Ok(value) => assert_eq!(10, value),
        Err(_) => unreachable!(),
    }
}
