use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    loop {
        let (socket, sock_addr) = listener.accept().await.unwrap();
        println!("accept sock_addr: {:?}", sock_addr);
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    let mut db = HashMap::new();
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);
        let resp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplement {:?}", cmd),
        };
        connection.write_frame(&resp).await.unwrap();
    }
}
