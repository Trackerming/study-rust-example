pub mod command_parser;
pub mod http_send_recv;
pub mod proto_helpers;

use command_parser::{ClientArgs, ServerArgs, TcpArgs};
use core::str::FromStr;
use nix::libc::listen;
use nix::sys::socket::{
    accept, bind, connect, listen as listen_vsock, shutdown, socket, AddressFamily, InetAddr,
    Shutdown, SockAddr, SockFlag, SockProtocol::Tcp, SockType,
};
use nix::unistd::close;
use proto_helpers::{recv_loop, recv_u64, send_loop, send_u64};
use std::convert::TryInto;
use std::ffi::c_int;
use std::net::{IpAddr, SocketAddr};
use std::os::unix::io::{AsRawFd, RawFd};
use std::string::String;

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
const BUF_MAX_LEN: usize = 8192;
const BACKLOG: usize = 128;
const MAX_CONNECTION_ATTEMPTS: usize = 5;

struct VsockSocket {
    socket_fd: RawFd,
}

impl VsockSocket {
    fn new(socket_fd: RawFd) -> Self {
        VsockSocket { socket_fd }
    }
}

// 实现drop特性，在结构体超出生命周期时候实现资源的关闭
impl Drop for VsockSocket {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|e| eprintln!("Failed to shut socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close socket: {:?}", e));
    }
}

impl AsRawFd for VsockSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

fn vsock_connect(cid: u32, port: u32) -> Result<VsockSocket, String> {
    // target os为android 和 linux
    let socket_addr = SockAddr::new_vsock(cid, port);
    let mut err_msg = String::new();
    for i in 0..MAX_CONNECTION_ATTEMPTS {
        let vsocket = VsockSocket::new(
            socket(
                AddressFamily::Vsock,
                SockType::Stream,
                SockFlag::empty(),
                None,
            )
            .map_err(|err| format!("Failed to create the socket: {:?}", err))?,
        );
        match connect(vsocket.as_raw_fd(), &socket_addr) {
            Ok(_) => return Ok(vsocket),
            Err(e) => err_msg = format!("Failed to connect: {}", e),
        }
        // 重连的时候显式降频
        std::thread::sleep(std::time::Duration::from_secs(1 << i));
    }
    Err(err_msg)
}

pub fn client(args: ClientArgs) -> Result<(), String> {
    let vsocket = vsock_connect(args.cid, args.port)?;
    let fd = vsocket.as_raw_fd();
    // 示例发送数据
    let data = "hello server, this is client".to_string();
    let buf = data.as_bytes();
    let len: u64 = buf.len().try_into().map_err(|err| format!("{:?}", err))?;
    send_u64(fd, len)?;
    send_loop(fd, buf, len)?;
    Ok(())
}

pub fn server(args: ServerArgs) -> Result<(), String> {
    let socket_fd = socket(
        AddressFamily::Vsock,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .map_err(|err| format!("server create socket failed: {:?}", err))?;
    let socket_addr = SockAddr::new_vsock(VMADDR_CID_ANY, args.port);
    bind(socket_fd, &socket_addr).map_err(|err| format!("server bind failed: {:?}", err))?;
    listen_vsock(socket_fd, BACKLOG).map_err(|err| format!("Listen failed: {:?}", err))?;
    loop {
        let fd = accept(socket_fd).map_err(|err| format!("Accept failed: {:?}", err))?;
        // server example
        let len = recv_u64(fd)?;
        let mut buf = [0u8; BUF_MAX_LEN];
        recv_loop(fd, &mut buf, len)?;
        println!(
            "{}",
            String::from_utf8(buf.to_vec())
                .map_err(|err| format!("The received bytes are not utf8: {:?}", err))?
        );
    }
}

pub fn http_server(args: TcpArgs) -> Result<(), String> {
    let socket_fd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        Tcp,
    )
    .map_err(|error| format!("sever create tcp socket failed: {:?}", error))?;
    let addr = SocketAddr::new(
        IpAddr::from_str(args.host).expect("ip address to Ipv4Addr failed."),
        args.port,
    );
    let socket_addr = SockAddr::Inet(InetAddr::from_std(&addr));
    bind(socket_fd, &socket_addr).map_err(|err| format!("tcp server bind failed: {:?}", err))?;
    unsafe {
        listen(socket_fd, BACKLOG as c_int);
    }
    loop {
        let fd = accept(socket_fd).map_err(|err| format!("tcp accept failed: {:?}", err))?;
        // server example
        let len = recv_u64(fd)?;
        let mut buf = [0u8; BUF_MAX_LEN];
        recv_loop(fd, &mut buf, len)?;
        println!(
            "{}",
            String::from_utf8(buf.to_vec())
                .map_err(|err| format!("The received bytes are not utf8: {:?}", err))?
        );
    }
}

struct TcpSocket {
    socket_fd: RawFd,
}

impl TcpSocket {
    fn new(socket_fd: RawFd) -> Self {
        TcpSocket { socket_fd }
    }
}

impl Drop for TcpSocket {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|e| eprintln!("Failed to shut tcp socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close tcp socket: {:?}", e));
    }
}

impl AsRawFd for TcpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

fn tcp_connect(host: &str, port: u16) -> Result<TcpSocket, String> {
    let addr = SocketAddr::new(
        IpAddr::from_str(host).expect("ip address to Ipv4Addr failed."),
        port,
    );
    let socket_addr = SockAddr::Inet(InetAddr::from_std(&addr));
    let mut err_msg = String::new();
    for i in 0..MAX_CONNECTION_ATTEMPTS {
        let tcp_socket = TcpSocket::new(
            socket(
                AddressFamily::Inet,
                SockType::Stream,
                SockFlag::empty(),
                Tcp,
            )
            .map_err(|err| format!("Failed to create the tcp socket: {:?}", err))?,
        );
        match connect(tcp_socket.as_raw_fd(), &socket_addr) {
            Ok(_) => return Ok(tcp_socket),
            Err(e) => err_msg = format!("Failed to connect: {}", e),
        }
        // 重连的时候显式降频
        std::thread::sleep(std::time::Duration::from_secs(1 << i));
    }
    Err(err_msg)
}
