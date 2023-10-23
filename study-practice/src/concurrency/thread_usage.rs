use std::cell::RefCell;
use std::sync::{Arc, Barrier, Condvar, Mutex, Once};
use std::thread;
use std::time::Duration;

fn example_basic_create() {
    let params = vec!["str", "rust", "hello", "world"];
    let handle = thread::spawn(move || {
        // 如果不用move移走所有权会出现错误：closure may outlive the current function, but it borrows `params`, which is owned by the current function
        for i in 0..5 {
            println!(
                "hi number {i} get param: {:?} from spawn thread.",
                params.get(i)
            );
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 阻塞主线程
    // handle.join().unwrap();
    for i in 0..4 {
        println!("hi number {i} from main thread.");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn example_block() {
    let thread_a = thread::spawn(|| {
        let inner_thread = thread::spawn(|| loop {
            println!("this is thread created in thread_a");
        });
        // inner_thread.join().unwrap();
    });
    thread_a.join().unwrap();
    println!("Child thread is finish");
    thread::sleep(Duration::from_secs(1));
}

fn thread_barrier() {
    let mut handles = Vec::with_capacity(5);
    let barrier = Arc::new(Barrier::new(5));
    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait...");
            b.wait();
            println!("after wait.");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
/// 线程局部变量
fn thread_local_example() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        })
    });
    t.join().unwrap();
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    })
}

/*
1. main 线程首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
2. 子线程获取到锁，并将其修改为 true，然后调用条件变量的 notify_one 方法来通知主线程继续执行
*/
fn condition_var_example() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    thread::spawn(move || {
        let (mutex_var, cond_var) = &*pair2;
        let mut started = mutex_var.lock().unwrap();
        println!("change started.");
        *started = true;
        cond_var.notify_one();
    });

    let (mutex_var, cond_var) = &*pair;
    let mut started = mutex_var.lock().unwrap();
    while !*started {
        started = cond_var.wait(started).unwrap();
    }
    println!("started changed");
}

fn call_once() {
    let mut val: RefCell<usize> = RefCell::new(0);
    let init_once = Arc::new(Once::new());
    let val1 = val.clone();
    let init_once1 = init_once.clone();
    let handle1 = thread::spawn(move || {
        init_once1.call_once(|| {
            *val1.borrow_mut() = 1;
            println!("handle1 thread val: {:?}.", val1);
        })
    });
    let val2 = val.clone();
    let init_once2 = init_once.clone();
    let handle2 = thread::spawn(move || {
        init_once2.call_once(|| {
            *val2.borrow_mut() = 2;
            println!("handle2 thread val: {:?}.", val2);
        })
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("after init val = {:?}", val);
}

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn call_once_static() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        })
    });
    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        })
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("after init val = {:?}", unsafe { VAL });
}

pub fn practice() {
    // example_basic_create();
    // example_block();
    // thread_barrier();
    // thread_local_example();
    // condition_var_example();
    call_once();
    call_once_static();
}
