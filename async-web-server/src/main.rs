use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task::{self, spawn};
use futures::stream::StreamExt;
use std::{fs, time::Duration};

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    // listener.incoming()是阻塞式的迭代器
    /*for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connect(stream).await;
    }*/
    listener
        .incoming()
        .for_each_concurrent(None, |tcp_stream| async move {
            let tcp_stream = tcp_stream.unwrap();
            spawn(handle_connect(tcp_stream));
        })
        .await;
}

async fn handle_connect(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        // 模拟慢请求，不用thread，thread会阻塞，当前的线程休眠，让其他任务无法继续
        // thread::sleep(Duration::from_secs(10));
        async_std::task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{content}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
