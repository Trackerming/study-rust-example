use crate::proto_helpers::{recv_loop, recv_u64, send_loop, send_u64};
use crate::proto_socket::{ProtoSocket, ProtoType};
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

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
const BACKLOG: usize = 128;
const BUF_MAX_LEN: usize = 8192;

pub struct SenderReceiver<'a> {
    send_socket: ProtoSocket<'a>,
    recv_proto_type: ProtoType<'a>,
    // 使用buf接收数据，当前只在单线程下；多线程可以考虑消息通信的方式
    buf: RefCell<[u8; BUF_MAX_LEN]>,
}

impl<'a> SenderReceiver<'a> {
    pub fn new_with_proto_type(
        send_proto_type: ProtoType<'a>,
        recv_proto_type: ProtoType<'a>,
    ) -> Self {
        SenderReceiver {
            send_socket: ProtoSocket::connect(send_proto_type).expect("send proto type error."),
            recv_proto_type,
            buf: RefCell::new([0u8; BUF_MAX_LEN]),
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
        Ok(())
    }

    fn send_data(&self, len: u64) -> Result<(), String> {
        let fd = self.send_socket.as_raw_fd();
        send_u64(fd, len)?;
        send_loop(fd, &*self.buf.borrow(), len)?;
        Ok(())
    }

    pub fn listen_sever(&self) -> Result<(), String> {
        let (raw_fd, socket_addr) = match self.recv_proto_type {
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
        // bind 和 listen
        bind(raw_fd, &socket_addr).map_err(|err| format!("server bind failed: {:?}.", err))?;
        self.listen_socket(raw_fd)?;
        // 接收数据
        loop {
            let fd = accept(raw_fd).map_err(|err| format!("server accept failed: {:?}", err))?;
            let len = recv_u64(fd)?;
            let buf = &mut *self.buf.borrow_mut();
            recv_loop(fd, buf, len)?;
            println!(
                "{}",
                String::from_utf8(self.buf.borrow().to_vec())
                    .map_err(|err| format!("The received bytes are not utf8: {:?}", err))?
            );
            self.send_data(len)?;
            // 清空数据
            buf.iter_mut().for_each(|e| *e = 0u8);
        }
    }
}
