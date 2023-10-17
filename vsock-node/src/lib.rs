pub mod command_parser;
pub mod create_app;
pub mod http_send_recv;
pub mod proto_helpers;
pub mod proto_socket;

use command_parser::{ClientArgs, ServerArgs, TcpArgs};
use core::str::FromStr;
use nix::libc::listen;
use nix::sys::socket::{
    accept, bind, listen as listen_vsock, socket, AddressFamily, InetAddr, SockAddr, SockFlag,
    SockProtocol::Tcp, SockType,
};
use proto_helpers::{recv_loop, recv_u64, send_loop, send_u64};
use proto_socket::{ProtoSocket, ProtoType};
use std::convert::TryInto;
use std::ffi::c_int;
// 显式引入trait到作用域
use std::net::{IpAddr, SocketAddr};
use std::os::unix::io::AsRawFd;
use std::string::String;

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
const BUF_MAX_LEN: usize = 8192;
const BACKLOG: usize = 128;

pub fn client(args: ClientArgs) -> Result<(), String> {
    let proto_type = ProtoType::Vsock(args.cid, args.port);
    let vsocket = ProtoSocket::connect(proto_type)?;
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

pub fn tcp_server(args: TcpArgs) -> Result<(), String> {
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

pub fn tcp_client(args: TcpArgs) -> Result<(), String> {
    let proto_type = ProtoType::Tcp(args.host, args.port);
    let tcp_socket = ProtoSocket::connect(proto_type)?;
    let fd = tcp_socket.as_raw_fd();
    // 示例发送数据
    let data = "hello server, this is tcp client".to_string();
    let buf = data.as_bytes();
    let len: u64 = buf.len().try_into().map_err(|err| format!("{:?}", err))?;
    send_u64(fd, len)?;
    send_loop(fd, buf, len)?;
    Ok(())
}
