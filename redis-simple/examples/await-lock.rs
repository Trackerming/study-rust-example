use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;

#[tokio::main]
async fn main() {
    let mutex_lock_value = Mutex::new(32);
    scope_fix(&mutex_lock_value).await;
    let can_inc = CanIncrement {
        value: Mutex::new(32),
    };
    do_inc_and_else(&can_inc).await;
    let tokio_mutex_lock = TokioMutex::new(64);
    do_with_tokio_lock(&tokio_mutex_lock).await;
}

// 方法1: 使用作用域的方式回收对应的lock
async fn scope_fix(mutex_lock_value: &Mutex<i32>) {
    //{
    let mut lock_value = mutex_lock_value.lock().unwrap();
    *lock_value += 1;
    println!("current value: {:?}", lock_value);
    //} // 这里mutex lock就被drop了
    // 可能会让出线程给其他的操作
    do_something_else_async().await;
}

// 方法2:将锁相关的操作封装为同步的代码结构
struct CanIncrement {
    value: Mutex<i32>,
}

impl CanIncrement {
    fn increment(&self) {
        let mut value_lock = self.value.lock().unwrap();
        *value_lock += 1;
        println!("current value: {:?}", value_lock);
    }
}

async fn do_inc_and_else(can_inc: &CanIncrement) {
    can_inc.increment();
    do_something_else_async().await;
}

// 方法3:采用tokio的lock实现
async fn do_with_tokio_lock(mutex_lock: &TokioMutex<i32>) {
    let mut lock_value = mutex_lock.lock().await;
    *lock_value += 64;
    println!("current value: {:?}", lock_value);
    do_something_else_async().await;
}

async fn do_something_else_async() {
    let handle = std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(1));
        println!("do something else async.");
    });
    handle.join().unwrap();
}
