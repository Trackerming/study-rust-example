use std::os::unix::net::{UnixListener, UnixStream};
use std::thread;

fn handle_client(stream: UnixStream) {
    println!("server stream: {:?}", stream);
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("/tmp/socket")?;
    match listener.accept() {
        Ok((socket, addr)) => println!("Got a client: {addr:?}"),
        Err(e) => println!("accept function failed: {e:?}"),
    }
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        }
    }
    Ok(())
}
