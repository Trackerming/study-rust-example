use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::TokioIo;
use serde_json;
use tokio::net::TcpStream;
use tracing::info;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub async fn fetch_url(uri: hyper::Uri, body_str: String) -> Result<()> {
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {:?}", err);
        }
    });
    let req = Request::post(uri)
        .header("Content-Type", "application/json")
        .header("Content-Len", body_str.len())
        .body(body_str)?;
    info!("Req: {:#?}", req);
    let mut res = sender.send_request(req).await?;
    info!("Res: {:?}", res);
    info!("Headers: {:#?}", res.headers());
    // asynchronously aggregate the chunks of the body
    let body = res.collect().await?.aggregate();
    // try to parse as json with serde_json
    let result = serde_json::from_reader(body.reader())?;
    info!("result: {:?}", result);
    Ok(())
}
