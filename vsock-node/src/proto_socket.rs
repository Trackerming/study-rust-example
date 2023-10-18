use core::str::FromStr;
use nix::sys::socket::{
    connect, shutdown, socket, AddressFamily, InetAddr, Shutdown, SockAddr, SockFlag,
    SockProtocol::Tcp, SockType,
};
use nix::unistd::close;
use std::net::{IpAddr, SocketAddr};
use std::os::unix::io::{AsRawFd, RawFd};
use std::string::String;

const MAX_CONNECTION_ATTEMPTS: usize = 5;

#[derive(Clone, Debug)]
pub(crate) enum ProtoType<'a> {
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

    fn get_raw_fd(proto_type: &ProtoType) -> Result<RawFd, String> {
        let raw_fd = match *proto_type {
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

    pub fn connect(proto_type: ProtoType<'a>) -> Result<ProtoSocket, String> {
        let addr = Self::get_addr(&proto_type);
        let mut err_msg = String::new();
        for i in 0..MAX_CONNECTION_ATTEMPTS {
            let raw_fd = Self::get_raw_fd(&proto_type)?;
            println!("SockAddr: {:?}, raw_fd: {:?}", addr, raw_fd);
            let proto_socket = ProtoSocket::new(proto_type.clone(), raw_fd);
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
