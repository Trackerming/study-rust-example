use std::sync::mpsc;
// multi-producer-single-consumer
use std::thread;
use std::time::Duration;

/// 运行结果如下
/// ```txt
/// main thread got: other thread send hi.
/// ```
fn channel_test() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("other thread send hi.");
        tx.send(val).unwrap();
        // error[E0382]: borrow of moved value: `val`
        // println!("other thread val: {}", val);
    });
    let received = rx.recv().unwrap();
    println!("main thread got: {received}");
}

/// 运行结果如下
/// ```txt
/// main thread got: other thread val 1
/// main thread got: other thread val 2
/// main thread got: other thread val 3
/// main thread got: other thread val 4
/// ```
fn send_multi_val_in_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("other thread val 1"),
            String::from("other thread val 2"),
            String::from("other thread val 3"),
            String::from("other thread val 4"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receives in rx {
        println!("main thread got: {}", receives);
    }
}

/// 运行结果如下
/// ```txt
/// main thread Got val: other thread val 1send from tx1
/// main thread Got val: send from tx other thread.
/// main thread Got val: other thread val 1send from tx2
/// main thread Got val: other thread val 2send from tx1
/// main thread Got val: other thread val 2send from tx2
/// main thread Got val: other thread val 3send from tx2
/// main thread Got val: other thread val 3send from tx1
/// main thread Got val: other thread val 4send from tx2
/// main thread Got val: other thread val 4send from tx1
/// ```
fn multi_producer_clone_test() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("other thread val 1"),
            String::from("other thread val 2"),
            String::from("other thread val 3"),
            String::from("other thread val 4"),
        ];
        for val in vals {
            tx1.send(val + "send from tx1").unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("other thread val 1"),
            String::from("other thread val 2"),
            String::from("other thread val 3"),
            String::from("other thread val 4"),
        ];
        for val in vals {
            tx2.send(val + "send from tx2").unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 不消耗完原始的tx，主线程会一直阻塞；
    thread::spawn(move || {
        tx.send(String::from("send from tx other thread.")).unwrap();
    });
    for receives in rx {
        println!("main thread Got val: {}", receives);
    }
}

pub fn messsage_thread_study() {
    channel_test();
    send_multi_val_in_channel();
    multi_producer_clone_test();
}
