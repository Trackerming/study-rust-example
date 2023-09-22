use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection_by_request_line(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    /*let mut status_line = String::from("HTTP/1.1 404 NOT FOUND");
    let mut content = String::new();
    if request_line == "GET / HTTP/1.1" {
        status_line = String::from("HTTP/1.1 200 OK");
        content = fs::read_to_string("hello.html").unwrap();
    } else {
        content = fs::read_to_string("404.html").unwrap();
    }*/
    /*let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request_line == "GET /sleep HTTP/1.1" {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };*/
    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection_by_request_line(stream);
        });

        println!("connection established.");
    }
}
