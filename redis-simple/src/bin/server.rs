use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8888";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listen on {:?}", addr);
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, sock_addr) = listener.accept().await.unwrap();
        let db = Arc::clone(&db);
        println!("accept sock_addr: {:?}", sock_addr);
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);
        let resp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplement {:?}", cmd),
        };
        connection.write_frame(&resp).await.unwrap();
    }
}
