use futures::executor::block_on;
use std::thread;
use std::time::Duration;

async fn do_first_thing() {
    // thread::sleep(Duration::from_secs(1));
    println!("async function do first thing: async async async");
}

async fn do_else_thing() {
    println!("async function do else thing: async async async");
}

async fn do_something() {
    // .await等待do_first_thing异步调用的完成，但是这个不会阻塞当前的线程；它是异步的等待Future的完成
    do_first_thing().await;
    do_else_thing().await;
    println!("async function do something: async async async");
}

async fn do_a() -> String {
    println!("do a");
    return String::from("after do a");
}

async fn do_b(param: String) {
    println!("{param}");
    println!("do b");
}

async fn do_a_and_b() {
    let ret = do_a().await;
    do_b(ret).await;
}

async fn do_c() {
    println!("do c");
}

async fn do_thing_main() {
    let do_1 = do_a_and_b();
    let do_2 = do_c();
    futures::join!(do_1, do_2);
}

pub fn practice() {
    // 使用一个执行器使用future:执行Future并等待其运行完成
    block_on(do_something());
    block_on(do_thing_main());
}
