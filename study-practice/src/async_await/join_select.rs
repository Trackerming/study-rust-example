use futures::executor::block_on;
use futures::stream::{FusedStream, Stream, StreamExt};
use futures::{
    future::{BoxFuture, FutureExt},
    pin_mut, select, try_join, TryFutureExt,
};
use std::fmt::Error as FmtError;
use std::future::{join, Future};
use std::io::Error;
use std::rc::Rc;

#[derive(Debug)]
struct Task<T> {
    name: String,
    value: T,
}

async fn task1() -> Result<Task<u32>, Error> {
    println!("running task 1");
    Ok(Task {
        name: "task1".to_string(),
        value: 38,
    })
}

async fn task2() -> Result<Task<String>, FmtError> {
    println!("running task 2");
    Ok(Task {
        name: "task2".to_string(),
        value: "random".to_string(),
    })
}

async fn task3() -> Result<Task<String>, FmtError> {
    println!("running task 3");
    Err(FmtError {})
}

async fn join_example() {
    println!("join example...");
    let task1_fut = task1();
    let task2_fut = task2();
    let (ret1, ret2) = join!(task1_fut, task2_fut).await;
    println!("ret1: {:?} ret2: {:?}", ret1, ret2);
    println!("try_join example...");
    let task1_fut = task1();
    let task3_fut = task3();
    let ret = try_join!(
        task3_fut.map_err(|err| "task1 error".to_string()),
        task1_fut.map_err(|err| "task3 error".to_string())
    );
    println!("ret: {:?}", ret);
}

/// join只能等所有的Future结束之后才能集中处理结果，如果想同时处理多个Future，可以尝试select
async fn select_example() {
    println!("select example...");
    let task1_fuse = task1().fuse();
    let task2_fuse = task2().fuse();
    pin_mut!(task1_fuse, task2_fuse);
    select! {
        task1_ret = task1_fuse => {
            println!("finish task1: {:?}", task1_ret);
        },
        task2_ret = task2_fuse => {
            println!("finish task2: {:?}", task2_ret);
        },
    }
}

async fn loop_select_example() {
    println!("loop select example...");
    // .fuse让Future实现FusedFuture特征
    // 当 Future 一旦完成后，那 select 就不能再对其进行轮询使用。
    // Fuse 意味着熔断，相当于 Future 一旦完成，再次调用 poll 会直接返回 Poll::Pending
    let task1_fuse = task1().fuse();
    let task2_fuse = task2().fuse();
    // 为Future实现Unpin特征，这里select不会通过拿走所有权的方式使用Future，是通过可变引用的方式使用的；这样当select结束之后，Future若没有完成，所有权还可以继续被其他代码使用
    pin_mut!(task1_fuse, task2_fuse);
    loop {
        select! {
            task1_ret = task1_fuse => {
                println!("finish task1: {:?}", task1_ret);
            },
            task2_ret = task2_fuse => {
                println!("finish task2: {:?}", task2_ret);
            },
            complete => break,
            default => panic!()
        }
    }
}

async fn add_two_stream(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;
    loop {
        let item = select! {
            s  = s1.next() => s,
            s  = s2.next() => s,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }
    total
}

/*async fn run_loop(
    mut timer_interval: impl Stream<Item=()> + FusedStream + Unpin,
    start_num: u8,
) {
    let task1_fuse = task1().fuse();
    let get_new_task_fut = Fuse::terminated();
    pin_mut!(task1_fuse, get_new_task_fut);
    loop {
        select! {
            () = timer_interval.select_next_some() => {
                //if get_new_task_fut.is_terminated(){
                    // get_new_task_fut.set(task1().fuse());
                //}
            }
        }
    }
}*/

/// 非send的特征的数据在async的作用域
#[derive(Default)]
struct NotSendTraitStruct(Rc<()>);

async fn bar() {
    println!("bar...");
}

async fn foo() {
    println!("foo");
    {
        let _not_send_struct = NotSendTraitStruct::default();
    } // 在这里not_send_struct被drop了
    bar().await;
}

fn require_send(_: impl Send) {}

/// 递归async fn
fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }
    .boxed()
}

/// trait中定义async fn
/*
error[E0706]: functions in traits cannot be declared `async`
   --> study-practice/src/async_await/join_select.rs:158:5
    |
158 |     async fn test();
    |     -----^^^^^^^^^^^
    |     |
    |     `async` because of this
    |
    = note: `async` trait functions are not currently supported
    = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
    = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
*/
trait TestAsync {
    async fn test();
}

pub fn practice() {
    block_on(join_example());
    block_on(select_example());
    block_on(loop_select_example());
    let fut = async {
        let _ = task1().await.map_err(|err| "err1".to_string());
        let _ = task2().await.map_err(|err| "err2".to_string());
    };
    block_on(fut);
    require_send(foo());
}
