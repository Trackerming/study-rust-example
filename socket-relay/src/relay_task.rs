use anyhow::Result;
use bytes::BytesMut;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tracing::{debug, info};

use crate::enclave_agnostic::enclave::{
    connect_to_enclave, shutdown_enclave_stream, EnclaveAddr, EnclaveStream,
};

pub(crate) struct RelayTask {
    src_conn: TcpStream,
    dest_conn: EnclaveStream,
    src_rx_bytes: BytesMut,
    dest_rx_bytes: BytesMut,
}

impl RelayTask {
    pub async fn new(
        src_conn: TcpStream,
        dest_addr: EnclaveAddr,
        buffer_size: usize,
    ) -> Result<Self> {
        let dest_conn = connect_to_enclave(dest_addr).await?;
        Ok(Self {
            src_conn,
            dest_conn,
            src_rx_bytes: BytesMut::with_capacity(buffer_size),
            dest_rx_bytes: BytesMut::with_capacity(buffer_size),
        })
    }

    async fn shutdown(&mut self) {
        self.src_conn.shutdown().await.ok();
        shutdown_enclave_stream(&mut self.dest_conn).await;
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
        self.src_conn.write_buf(&mut self.dest_rx_bytes).await?;
        Ok(true)
    }

    async fn handle_src_conn_rx(&mut self, rx_result: io::Result<usize>) -> Result<bool> {
        if !self.handle_rx_result(rx_result).await? {
            debug!("recv empty buf from src connection, quitting comm");
            return Ok(false);
        }
        info!("handle src conn rx bytes len: {}", self.src_rx_bytes.len());
        self.dest_conn.write_buf(&mut self.src_rx_bytes).await?;
        Ok(true)
    }

    pub async fn run(mut self) -> Result<()> {
        let mut should_continue = true;
        while should_continue {
            should_continue = tokio::select! {
                rslt = self.src_conn.read_buf(&mut self.src_rx_bytes) => self.handle_src_conn_rx(rslt).await,
                result = self.dest_conn.read_buf(&mut self.dest_rx_bytes) => self.handle_dest_conn_rx(result).await,
            }?;
        }
        Ok(())
    }
}
