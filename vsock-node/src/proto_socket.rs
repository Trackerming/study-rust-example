use core::str::FromStr;
use libc::{suseconds_t, time_t};
use nix::sys::socket::{
    connect, shutdown, socket, AddressFamily, InetAddr, Shutdown, SockAddr, SockFlag,
    SockProtocol::Tcp, SockType,
};
use nix::unistd::close;
use std::net::{IpAddr, SocketAddr};
use std::os::unix::io::{AsRawFd, RawFd};
use std::string::String;

extern crate libc;

const MAX_CONNECTION_ATTEMPTS: usize = 5;

#[derive(Clone, Debug)]
pub enum ProtoType<'a> {
    Vsock(u32, u32),
    Tcp(&'a str, u16),
}

#[derive(Debug)]
pub struct ProtoSocket<'a> {
    proto_type: ProtoType<'a>,
    socket_fd: RawFd,
}

impl<'a> ProtoSocket<'a> {
    pub fn new(proto_type: ProtoType<'a>, socket_fd: RawFd) -> ProtoSocket {
        Self {
            proto_type,
            socket_fd,
        }
    }

    fn get_addr(proto_type: &ProtoType) -> SockAddr {
        match *proto_type {
            ProtoType::Vsock(cid, port) => SockAddr::new_vsock(cid, port),
            ProtoType::Tcp(host, tcp_port) => {
                let addr = SocketAddr::new(
                    IpAddr::from_str(host).expect("ip address to Ipv4Addr failed."),
                    tcp_port,
                );
                SockAddr::Inet(InetAddr::from_std(&addr))
            }
        }
    }

    fn get_raw_fd(proto_type: ProtoType) -> Result<RawFd, String> {
        let raw_fd = match proto_type {
            ProtoType::Vsock(_, _) => {
                println!("get_raw_fd ProtoType::Vsock");
                // target os为android 和 linux
                socket(
                    AddressFamily::Vsock,
                    SockType::Stream,
                    SockFlag::empty(),
                    None,
                )
                .map_err(|err| format!("Failed to create the socket: {:?}", err))?
            }
            ProtoType::Tcp(_, _) => {
                println!("get_raw_fd ProtoType::Tcp");
                socket(
                    AddressFamily::Inet,
                    SockType::Stream,
                    SockFlag::empty(),
                    Tcp,
                )
                .map_err(|err| format!("Failed to create the tcp socket: {:?}", err))?
            }
        };
        Ok(raw_fd)
    }

    fn set_timeout(tv_sec: u32, tc_usec: u32, raw_fd: RawFd) -> bool {
        // 创建一个 `libc::timeval` 结构来设置超时时间
        let mut timeout = libc::timeval {
            tv_sec: tv_sec as time_t,        // 秒
            tv_usec: tc_usec as suseconds_t, // 微秒 (1 秒 = 1,000,000 微秒)
        };
        // 设置连接超时选项
        let result = unsafe {
            libc::setsockopt(
                raw_fd,
                libc::SOL_SOCKET,
                libc::SO_RCVTIMEO,
                &timeout as *const _ as *const libc::c_void,
                std::mem::size_of_val(&timeout) as u32,
            )
        };
        if result != 0 {
            false
        } else {
            true
        }
    }

    pub fn connect(proto_type: ProtoType<'a>) -> Result<ProtoSocket, String> {
        let addr = Self::get_addr(&proto_type);
        let mut err_msg = String::new();
        for i in 0..MAX_CONNECTION_ATTEMPTS {
            let raw_fd = Self::get_raw_fd(proto_type.clone())?;
            println!("SockAddr: {:?}, raw_fd: {:?}", addr, raw_fd);
            let proto_socket = ProtoSocket::new(proto_type.clone(), raw_fd);
            Self::set_timeout(20, 0, proto_socket.as_raw_fd());
            match connect(proto_socket.as_raw_fd(), &addr) {
                Ok(_) => {
                    println!("connect proto_socket {:?}", proto_socket);
                    return Ok(proto_socket);
                }
                Err(e) => err_msg = format!("failed to connect {}", e),
            }
            // 重连的时候显式降频
            std::thread::sleep(std::time::Duration::from_secs(1 << i));
        }
        Err(err_msg)
    }
}

// 实现drop特性，在结构体超出生命周期时候实现资源的关闭
impl<'a> Drop for ProtoSocket<'a> {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|e| eprintln!("Failed to shut socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close socket: {:?}", e));
    }
}

impl<'a> AsRawFd for ProtoSocket<'a> {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}
