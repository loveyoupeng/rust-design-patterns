use criterion::{Criterion, criterion_group, criterion_main};
use rust_design_patterns::pub_sub::create_buffer;
use std::{
    hint::black_box,
    sync::{Arc, atomic::AtomicBool},
    thread,
};

fn pub_sub_benchmark(c: &mut Criterion) {
    let (publisher, subscriber) = create_buffer::<i32>(1024);
    let lock = Arc::new(AtomicBool::new(true));
    let inner_lock = lock.clone();
    let thread = thread::spawn(move || {
        while inner_lock.load(std::sync::atomic::Ordering::Acquire) {
            subscriber.try_poll();
        }
    });
    c.bench_function("publish", |b| {
        b.iter(|| while !publisher.try_offer(black_box(1)) {})
    });
    lock.store(false, std::sync::atomic::Ordering::Release);
    thread.join().unwrap();
}

criterion_group!(pub_sub, pub_sub_benchmark);
criterion_main!(pub_sub);
