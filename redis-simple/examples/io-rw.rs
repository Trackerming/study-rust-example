use tokio::fs::File;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    read_file_cont().await.unwrap();
    read_file_end().await.unwrap();
    copy_example().await.unwrap();
    // return_self_sever().await.unwrap();
    io_copy_return_self_server().await.unwrap();
}

async fn read_file_cont() -> io::Result<()> {
    let mut f = File::open("./examples/hello-redis.rs").await?;
    let mut buffer = [0; 10];
    loop {
        let n = f.read(&mut buffer[..]).await;
        if let Ok(0) = n {
            break;
        }
        println!("the bytes: {:?}", &buffer[..]);
    }
    Ok(())
}

async fn read_file_end() -> io::Result<String> {
    let mut f = File::open("./examples/hello-redis.rs").await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    let content = String::from_utf8(buffer).unwrap();
    println!("the bytes: {:?}", &content);
    Ok(content)
}

async fn copy_example() -> io::Result<()> {
    let mut content = read_file_end().await.unwrap();
    let mut file = File::create("test.txt").await?;
    io::copy(&mut content.as_bytes(), &mut file).await?;
    Ok(())
}

async fn return_self_sever() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6666").await?;
    let (mut read_stream, mut write_stream) = io::split(socket);
    tokio::spawn(async move {
        write_stream.write_all(b"hello\r\n").await?;
        write_stream.write_all(b"world\r\n").await?;
        Ok::<_, io::Error>(())
    });
    let mut buf = vec![0; 1024];
    loop {
        let n = read_stream.read(&mut buf).await;
        if let Ok(0) = n {
            break;
        }
        println!("Got: {:?}", &buf[..]);
    }
    Ok(())
}

async fn io_copy_return_self_server() -> io::Result<()> {
    let mut socket = TcpStream::connect("127.0.0.1:6666").await?;
    tokio::spawn(async move {
        let (mut read_stream, mut write_stream) = socket.split();
        if let Err(e) = io::copy(&mut read_stream, &mut write_stream).await {
            eprintln!("failed to copy {:?}", e);
        }
    });
    Ok(())
}
