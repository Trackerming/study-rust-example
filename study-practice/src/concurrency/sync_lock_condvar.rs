use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn mutex_usage_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for i in 0..8 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            let mut cnt = counter_clone.lock().unwrap();
            *cnt += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap());
}

fn rw_lock_example() {
    let int_rw_lock = RwLock::new(5);
    // 多次读取可以被允许
    {
        let r1 = int_rw_lock.read().unwrap();
        let r2 = int_rw_lock.read().unwrap();
        println!("r1: {:?} r2: {:?}", r1, r2);
    } // drop读锁
      // 同一时间只允许一次写
    {
        let mut w = int_rw_lock.write().unwrap();
        *w *= 2;
        println!("w: {:?}", w);
        // let r = int_rw_lock.read().unwrap();
        // println!("r: {:?} ", r);
    } // drop 写锁
}

fn cons_control_sync() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();
    let handle = thread::spawn(move || {
        let mut cflag_lock = cflag.lock().unwrap();
        let mut counter = 0;
        while counter < 3 {
            while !*cflag_lock {
                // wait方法接收一个MutexGuard<'a,T>,且会自动的暂时释放这个锁，其他线程可以拿到锁比ing进行线程的更新
                // 当前线程同时会在这里被阻塞，直到其他地方nitify之后，将原本的MutexGuard还给当前线程，即当前线程重新获取了锁
                cflag_lock = ccond.wait(cflag_lock).unwrap();
            }
            *cflag_lock = false;
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });
    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    handle.join().unwrap();
    println!("finish flag{:?}", flag);
}

pub fn practice() {
    mutex_usage_example();
    rw_lock_example();
    cons_control_sync();
}
