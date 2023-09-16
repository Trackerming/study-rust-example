use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

/// 运行结果：死锁
/// ```txt
/// thread1 handle counter num 110
/// thread2 handle counter num_b 260
/// ...
/// ```
fn dead_lock_create() {
    let counter = Arc::new(Mutex::new(100));
    let counter_b = Arc::new(Mutex::new(200));
    let mut handles = vec![];
    let counter_clone1 = counter.clone();
    let counter_clone_b = counter_b.clone();
    let handle1 = thread::spawn(move || {
        let mut num = counter_clone1.lock().unwrap();
        *num += 10;
        println!("thread1 handle counter num {}", num);
        thread::sleep(Duration::from_secs(1));
        let mut num_b = counter_clone_b.lock().unwrap();
        *num_b += 40;
        println!("thread1 handle counter num_b {}", num_b);
    });
    handles.push(handle1);
    let counter_clone2 = counter.clone();
    let counter_clone2_b = counter_b.clone();
    // 相反顺序获取锁
    let handle2 = thread::spawn(move || {
        let mut num_b = counter_clone2_b.lock().unwrap();
        *num_b += 60;
        thread::sleep(Duration::from_secs(1));
        println!("thread2 handle counter num_b {}", num_b);
        let mut num = counter_clone2.lock().unwrap();
        *num += 20;
        println!("thread2 handle counter num {}", num);
    });
    handles.push(handle2);
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap());
}

pub fn dead_lock_study() {
    dead_lock_create();
}
