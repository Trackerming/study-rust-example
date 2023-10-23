use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn example_for_receive() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec!["hello", "rust", "world"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    for r_str in rx.iter().enumerate() {
        println!("Got msg: {:?}", r_str);
    }
}

fn example_multi_sender() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        tx.send("this from thread 1.").unwrap();
    });
    thread::spawn(move || {
        tx2.send("this from thread 2.").unwrap();
    });
    for msg in rx {
        println!("Got msg: {:?}", msg);
    }
}

/// 同步通道
fn sync_channel() {
    // 参数是缓冲消息条数
    let (tx, rx) = mpsc::sync_channel(0);
    thread::spawn(move || {
        println!("before send...");
        tx.send(1).unwrap();
        println!("after send.")
    });
    println!("main thread before sleep...");
    thread::sleep(Duration::from_secs(3));
    println!("main thread after sleep.");
    println!("Get msg: {:?}", rx.recv().unwrap());
}

/// 异步非阻塞通道，消息发送了，也可以后续接收
fn async_channel() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        println!("before send...");
        tx.send(1).unwrap();
        println!("after send.")
    });
    println!("main thread before sleep...");
    thread::sleep(Duration::from_secs(3));
    println!("main thread after sleep.");
    println!("Get msg: {:?}", rx.recv().unwrap());
}

fn sync_async_channel() {
    sync_channel();
    async_channel();
}

pub fn practice() {
    // example_for_receive();
    // example_multi_sender();
    sync_async_channel();
}
