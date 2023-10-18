pub mod command_parser;
pub mod create_app;
pub mod http_send_recv;
pub mod proto_helpers;
pub mod proto_socket;
pub mod sender_receiver;

use crate::sender_receiver::SenderReceiver;
use command_parser::{ClientArgs, ServerArgs, TcpArgs};
use core::str::FromStr;
use proto_helpers::{send_loop, send_u64};
use proto_socket::{ProtoSocket, ProtoType};
use std::convert::TryInto;
use std::os::unix::io::AsRawFd;
use std::string::String;
const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;

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

pub fn vsock_to_tcp(args: ServerArgs, tcp_args: TcpArgs) -> Result<(), String> {
    let send_proto_type = ProtoType::Tcp(tcp_args.host, tcp_args.port);
    let recv_proto_type = ProtoType::Vsock(VMADDR_CID_ANY, args.port);
    let sender_receiver = SenderReceiver::new_with_proto_type(send_proto_type, recv_proto_type);
    sender_receiver.listen_sever()
}

pub fn tcp_to_vsock(args: TcpArgs, client_args: ClientArgs) -> Result<(), String> {
    let recv_proto_type = ProtoType::Tcp(args.host, args.port);
    let send_proto_type = ProtoType::Vsock(client_args.cid, client_args.port);
    let sender_receiver = SenderReceiver::new_with_proto_type(send_proto_type, recv_proto_type);
    sender_receiver.listen_sever()
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
