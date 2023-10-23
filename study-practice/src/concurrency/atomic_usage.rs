use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::{hint, thread};

struct Counter {
    count: u64,
}

fn atomic_usage_example() {
    let n = Mutex::new(Counter { count: 0 });
    n.lock().unwrap().count += 1;
    let n = AtomicU64::new(0);
    // Ordering::Relaxed 控制原子操作使用的内存顺序
    n.fetch_add(0, Ordering::Relaxed);
}

/// 内存屏障
static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;
        }
        // 保证之前的操作永远在之前
        READY.store(true, Ordering::Release);
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        // 保证之后的操作永远在之后
        while !READY.load(Ordering::Acquire) {}
        println!("in consumer thread: DATA: {:?}", unsafe { DATA });
    })
}

fn mem_barrier() {
    reset();
    let proc_handle = producer();
    let cons_handle = consumer();
    proc_handle.join().unwrap();
    cons_handle.join().unwrap();
}

fn atomic_usage_multi_thread() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = spinlock.clone();
    let handle1 = thread::spawn(move || {
        spinlock_clone.store(20, Ordering::SeqCst);
    });
    while spinlock.load(Ordering::SeqCst) != 20 {
        hint::spin_loop();
    }
    if let Err(e) = handle1.join() {
        println!("Thread had an error: {:?}", e);
    }
}

pub fn practice() {
    atomic_usage_example();
    mem_barrier();
    atomic_usage_multi_thread();
}
