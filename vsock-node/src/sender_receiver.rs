use crate::proto_helpers::{recv_loop, recv_u64, send_loop, send_u64};
use crate::proto_socket::{ProtoSocket, ProtoType};
use crate::thread_pool::ThreadPool;
use core::str::FromStr;
use nix::libc::listen;
use nix::sys::socket::{
    accept, bind, listen as listen_vsock, socket, AddressFamily, InetAddr, SockAddr, SockFlag,
    SockProtocol::Tcp, SockType,
};
use std::cell::RefCell;
use std::ffi::c_int;
use std::net::{IpAddr, SocketAddr};
use std::os::fd::{AsRawFd, RawFd};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

const BACKLOG: usize = 128;
const BUF_MAX_LEN: usize = 8192;

pub struct SenderReceiver<'a> {
    send_socket: ProtoSocket<'a>,
    recv_proto_type: ProtoType<'a>,
    chann: (
        Arc<Sender<(Vec<u8>, u64)>>,
        Arc<Mutex<Receiver<(Vec<u8>, u64)>>>,
    ),
}

fn send_data(fd: RawFd, rx: Arc<Mutex<Receiver<(Vec<u8>, u64)>>>) {
    let recv = rx.lock().unwrap().recv();
    match recv {
        Ok((buf, len)) => {
            println!(
                "in send data len: {len}, received len: {} {:?}",
                buf.len(),
                buf
            );
            send_u64(fd, len).expect("send data len failed");
            send_loop(fd, &buf, len).expect("send data failed.");
            println!("send_data finish.");
        }
        Err(e) => {
            eprintln!("recv chann error: {e}");
        }
    }
}

fn received_data(raw_fd: RawFd, tx: Arc<Sender<(Vec<u8>, u64)>>) {
    let fd = accept(raw_fd)
        .map_err(|err| eprintln!("server accept failed: {:?}", err))
        .unwrap();
    let len = recv_u64(fd).unwrap();
    println!("in receive data len: {len}");
    let mut buf: Vec<u8> = vec![0u8; len as usize];
    recv_loop(fd, &mut buf, len).unwrap();
    println!(
        "{}",
        String::from_utf8(buf.to_vec())
            .map_err(|err| eprintln!("The received bytes are not utf8: {:?}", err))
            .unwrap()
    );
    println!("received len: {:?}", buf);
    tx.send((buf, len)).expect("received data and send failed.");
}

impl<'a> SenderReceiver<'a> {
    pub fn new_with_proto_type(
        send_proto_type: ProtoType<'a>,
        recv_proto_type: ProtoType<'a>,
    ) -> Self {
        let (tx, rx) = mpsc::channel();
        SenderReceiver {
            send_socket: ProtoSocket::connect(send_proto_type).expect("send proto type error."),
            recv_proto_type,
            chann: (Arc::new(tx), Arc::new(Mutex::new(rx))),
        }
    }

    fn listen_socket(&self, raw_fd: RawFd) -> Result<(), String> {
        match self.recv_proto_type {
            ProtoType::Tcp(_, _) => unsafe {
                listen(raw_fd, BACKLOG as c_int);
            },
            ProtoType::Vsock(_, _) => {
                listen_vsock(raw_fd, BACKLOG).map_err(|err| format!("listen failed: {:?}", err))?;
            }
        };
        println!("sender_receiver listen socket finished.");
        Ok(())
    }

    pub fn listen_sever(&self) -> Result<(), String> {
        let (raw_fd, socket_addr): (RawFd, SockAddr) = match self.recv_proto_type {
            ProtoType::Vsock(cid, port) => {
                let socket_fd = socket(
                    AddressFamily::Vsock,
                    SockType::Stream,
                    SockFlag::empty(),
                    None,
                )
                .map_err(|err| format!("server create v-socket failed: {:?}", err))?;
                let socket_addr = SockAddr::new_vsock(cid, port);
                (socket_fd, socket_addr)
            }
            ProtoType::Tcp(host, port) => {
                let socket_fd = socket(
                    AddressFamily::Inet,
                    SockType::Stream,
                    SockFlag::empty(),
                    Tcp,
                )
                .map_err(|error| format!("sever create tcp socket failed: {:?}", error))?;
                let addr = SocketAddr::new(
                    IpAddr::from_str(host).expect("ip address to Ipv4Addr failed."),
                    port,
                );
                let socket_addr = SockAddr::Inet(InetAddr::from_std(&addr));
                (socket_fd, socket_addr)
            }
        };
        println!(
            "listen in raw_fd: {:?}, socket_addr: {:?}",
            raw_fd, socket_addr
        );
        // bind 和 listen
        bind(raw_fd, &socket_addr).map_err(|err| format!("server bind failed: {:?}.", err))?;
        self.listen_socket(raw_fd)?;
        let send_raw_fd = self.send_socket.as_raw_fd();
        loop {
            let tx_clone = self.chann.0.clone();
            let rx_clone = Arc::clone(&self.chann.1);
            // 接收数据
            received_data(raw_fd, tx_clone);
            send_data(send_raw_fd, rx_clone);
        }
    }
}
