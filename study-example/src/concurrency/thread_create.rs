use std::thread;
use std::time::Duration;

/// 运行结果如下
/// ```txt
/// main thread number 1
/// thread::spawn number 1
/// thread::spawn number 2
/// main thread number 2
/// thread::spawn number 3
/// main thread number 3
/// thread::spawn number 4
/// main thread number 4
/// thread::spawn number 5
/// ```
fn create_multi_thread_task() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("thread::spawn number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/// 运行结果如下
/// ```txt
/// main thread number 1
/// thread::spawn number 1
/// main thread number 2
/// thread::spawn number 2
/// thread::spawn number 3
/// main thread number 3
/// thread::spawn number 4
/// main thread number 4
/// thread::spawn number 5
/// thread::spawn number 6
/// thread::spawn number 7
/// thread::spawn number 8
/// thread::spawn number 9
/// ```
fn create_multi_thread_task_all_done() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread::spawn number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn move_to_clousure_with_thread() {
    let v = vec![1, 23, 456];
    // error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // Rust 无法判断生成的线程将运行多长时间，因此它不知道对 v 的引用是否始终有效，采用move强制闭包获取使用值的所有权
    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    // error[E0382]: use of moved value: `v`
    // drop(v);
    handle.join().unwrap();
}

pub fn thread_create_study() {
    create_multi_thread_task();
    create_multi_thread_task_all_done();
    move_to_clousure_with_thread();
}
