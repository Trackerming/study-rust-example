use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    my_select().await;
    loop_select_example().await;
}

async fn my_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();
    tx1.send("hello tx1").unwrap();
    MySelect { rx1, rx2 }.await;
}

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }
        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }
        Poll::Pending
    }
}

async fn action(input: Option<i32>) -> Option<String> {
    let i = match input {
        Some(input) => input,
        None => return None,
    };
    println!("in action {i}");
    Some("action finish i".to_string())
}

async fn loop_select_example() {
    let (mut tx, mut rx) = tokio::sync::mpsc::channel(128);
    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(2).await;
        let _ = tx.send(3).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(v) = res {
                    println!("Got {}", v);
                    return;
                }
            },
            Some(v) = rx.recv() => {
                if v%2 ==0 {
                    // Pin上定义的方法
                    operation.set(action(Some(v)));
                    done = false;
                }
            },
        }
    }
}
