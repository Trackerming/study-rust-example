use anyhow::Result;
use bytes::BytesMut;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{debug, info};

use crate::enclave_agnostic::enclave::{shutdown_enclave_stream, EnclaveStream};

#[derive(Debug)]
pub(crate) enum ConnectionStream {
    EnclaveStreamType(EnclaveStream),
    TcpStreamType(TcpStream),
}

#[derive(Debug)]
pub(crate) struct RelayTask {
    src_conn: ConnectionStream,
    dest_conn: ConnectionStream,
    src_rx_bytes: BytesMut,
    dest_rx_bytes: BytesMut,
}

async fn shutdown_connect_stream(connect_stream: &mut ConnectionStream) {
    let _ = match connect_stream {
        ConnectionStream::EnclaveStreamType(stream) => shutdown_enclave_stream(stream).await,
        ConnectionStream::TcpStreamType(stream) => stream
            .shutdown()
            .await
            .ok()
            .expect("shutdown tcp stream failed."),
    };
}

async fn read_from_stream(
    connect_stream: &mut ConnectionStream,
    buf: &mut BytesMut,
) -> io::Result<usize> {
    match connect_stream {
        ConnectionStream::EnclaveStreamType(stream) => stream.read_buf(buf).await,
        ConnectionStream::TcpStreamType(stream) => stream.read_buf(buf).await,
    }
}

impl RelayTask {
    pub async fn new(
        src_conn: ConnectionStream,
        dest_conn: ConnectionStream,
        buffer_size: usize,
    ) -> Result<Self> {
        Ok(Self {
            src_conn,
            dest_conn,
            src_rx_bytes: BytesMut::with_capacity(buffer_size),
            dest_rx_bytes: BytesMut::with_capacity(buffer_size),
        })
    }

    async fn shutdown(&mut self) {
        debug!("begin shutdown...");
        shutdown_connect_stream(&mut self.src_conn).await;
        shutdown_connect_stream(&mut self.dest_conn).await;
    }

    async fn handle_rx_result(&mut self, rx_result: io::Result<usize>) -> Result<bool> {
        match rx_result {
            Ok(bytes_read) => {
                // If bytes_read == 0, assume connection has terminated
                if bytes_read == 0 {
                    self.shutdown().await;
                    return Ok(false);
                }
            }
            Err(err) => {
                self.shutdown().await;
                return Err(err.into());
            }
        }
        Ok(true)
    }

    async fn handle_dest_conn_rx(&mut self, rx_result: io::Result<usize>) -> Result<bool> {
        if !self.handle_rx_result(rx_result).await? {
            debug!("recv empty buf from dest connection, quitting comm");
            return Ok(false);
        }
        info!(
            "handle dest conn rx bytes len: {}",
            self.dest_rx_bytes.len()
        );
        let _ = match &mut self.src_conn {
            ConnectionStream::TcpStreamType(stream) => {
                stream.write_buf(&mut self.dest_rx_bytes).await?
            }
            ConnectionStream::EnclaveStreamType(stream) => {
                stream.write_buf(&mut self.dest_rx_bytes).await?
            }
        };
        Ok(true)
    }

    async fn handle_src_conn_rx(&mut self, rx_result: io::Result<usize>) -> Result<bool> {
        if !self.handle_rx_result(rx_result).await? {
            debug!("recv empty buf from src connection, quitting comm");
            return Ok(false);
        }
        info!("handle src conn rx bytes len: {}", self.src_rx_bytes.len());
        let _ = match &mut self.dest_conn {
            ConnectionStream::TcpStreamType(stream) => {
                stream.write_buf(&mut self.src_rx_bytes).await?
            }
            ConnectionStream::EnclaveStreamType(stream) => {
                stream.write_buf(&mut self.src_rx_bytes).await?
            }
        };
        Ok(true)
    }

    pub async fn run(mut self) -> Result<()> {
        let mut should_continue = true;
        debug!("run task");
        while should_continue {
            should_continue = tokio::select! {
                result = read_from_stream(&mut self.src_conn, &mut self.src_rx_bytes) => {
                    debug!("read from src_conn stream, {:?}", self.src_conn);
                    self.handle_src_conn_rx(result).await
                },
                result = read_from_stream(&mut self.dest_conn, &mut self.dest_rx_bytes) => {
                    debug!("read from dest_conn stream {:?}", self.dest_conn);
                    self.handle_dest_conn_rx(result).await
                },
            }?;
        }
        Ok(())
    }
}
