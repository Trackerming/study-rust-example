use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// 运行结果如下
/// ```txt
/// m: Mutex { data: 8, poisoned: false, .. }
/// ```
fn mutex_api_usage() {
    let m = Mutex::new(42);
    {
        let mut num = m.lock().unwrap();
        *num = 8;
    }
    println!("m: {:?}", m);
}

/// 运行结果如下
/// ```txt
/// result: 10
/// ```
fn share_mutex_in_multi_thread() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap());
}

pub fn mutex_shared_state_study() {
    mutex_api_usage();
    share_mutex_in_multi_thread();
}
