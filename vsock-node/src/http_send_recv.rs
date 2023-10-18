use core::str::FromStr;
use std::io::BufReader;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};

pub struct HttpSender {
    stream: TcpStream,
}

pub struct HttpReceiver {
    listener: TcpListener,
}

impl HttpReceiver {
    fn new_with(host: &str, port: u16) -> HttpReceiver {
        let addr = SocketAddrV4::new(
            Ipv4Addr::from_str(host).expect("ip address to Ipv4Addr failed."),
            port,
        );
        let listener = TcpListener::bind(addr).expect("bind address failed.");
        HttpReceiver { listener }
    }

    fn receive_data(&self) -> Vec<u8> {
        let mut data = vec![0];
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let buf_reader = BufReader::new(&mut stream);
            data = buf_reader.buffer().to_vec();
        }
        return data;
    }
}
