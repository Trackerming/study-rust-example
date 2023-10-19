use byteorder::{ByteOrder, LittleEndian};
use nix::sys::socket::{recv, send, MsgFlags};
use std::mem::size_of;
use std::os::unix::io::RawFd;

pub fn send_u64(fd: RawFd, val: u64) -> Result<(), String> {
    let mut buf = Vec::with_capacity(size_of::<u64>());
    LittleEndian::write_u64(&mut buf, val);
    send_loop(fd, &buf, size_of::<u64>().try_into().unwrap())?;
    Ok(())
}

pub fn send_loop(fd: RawFd, buf: &Vec<u8>, len: u64) -> Result<(), String> {
    let len: usize = len.try_into().map_err(|err| format!("{:?}", err))?;
    let mut send_bytes = 0;
    while send_bytes < len {
        let size = match send(fd, &buf[send_bytes..len], MsgFlags::empty()) {
            Ok(size) => size,
            Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => 0,
            Err(err) => return Err(format!("{:?}", err)),
        };
        send_bytes += size;
    }
    Ok(())
}

pub fn recv_u64(fd: RawFd) -> Result<u64, String> {
    let mut buf = Vec::with_capacity(size_of::<u64>());
    recv_loop(fd, &mut buf, size_of::<u64>().try_into().unwrap())?;
    let val = LittleEndian::read_u64(&buf);
    Ok(val)
}

pub fn recv_loop(fd: RawFd, buf: &mut Vec<u8>, len: u64) -> Result<(), String> {
    let len: usize = len.try_into().map_err(|err| format!("{:?}", err))?;
    let mut recv_bytes = 0;
    while recv_bytes < len {
        let size = match recv(fd, &mut buf[recv_bytes..len], MsgFlags::empty()) {
            Ok(size) => size,
            Err(nix::Error::Sys(nix::errno::Errno::EINTR)) => 0,
            Err(err) => return Err(format!("{:?}", err)),
        };
        recv_bytes += size;
    }
    Ok(())
}
