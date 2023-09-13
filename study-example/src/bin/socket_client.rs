use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/socket")?;
    stream.write_all(b"hello world")?;
    let mut res = String::new();
    stream.read_to_string(&mut res)?;
    println!("{res}");
    Ok(())
}
