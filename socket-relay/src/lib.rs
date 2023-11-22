use anyhow::{Context, Result};
use clap::{command, Parser};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::{net::TcpListener, sync::Semaphore};
use tracing::{error, info};

use crate::enclave_agnostic::enclave::{
    connect_to_enclave, get_listener_server, parse_enclave_addr, DEFAULT_DEST_ADDR,
};
use crate::relay_task::{ConnectionStream, RelayTask};

pub(crate) mod enclave_agnostic;

pub(crate) mod relay_task;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(long, default_value_t = 8192)]
    buffer_size: usize,

    #[arg(short = 's', long, default_value = "0.0.0.0:8443")]
    tcp_address: String,

    #[arg(short = 'd', long, default_value = DEFAULT_DEST_ADDR)]
    vsock_address: String,

    #[arg(short = 'c', long, default_value_t = 1024)]
    max_concurrent_connections: usize,

    #[arg(short = 't', long)]
    tcp_to_vsock: bool,
}

pub async fn listen_vsock(args: &Cli) -> Result<()> {
    let address = parse_enclave_addr(&args.vsock_address)?;
    let enclave_listener = get_listener_server(address).await?;
    let destination_address =
        SocketAddr::from_str(&args.tcp_address).context("parse tcp address failed.")?;
    let conn_count_semaphore = Arc::new(Semaphore::new(args.max_concurrent_connections));
    info!("Listening on vsock {}...", args.vsock_address);

    while let Ok(semaphore_permit) = conn_count_semaphore.clone().acquire_owned().await {
        match enclave_listener.accept().await {
            Ok((enclave_stream, _)) => {
                let buf_size = args.buffer_size;

                // Spawn new task to handle connection, task will now own semaphore
                // for the duration of the connection.
                tokio::spawn(async move {
                    let result = async {
                        let tcp_stream = TcpStream::connect(destination_address).await?;
                        let task = RelayTask::new(
                            ConnectionStream::EnclaveStreamType(enclave_stream),
                            ConnectionStream::TcpStreamType(tcp_stream),
                            buf_size,
                        )
                        .await?;
                        task.run().await
                    };
                    if let Err(e) = result.await {
                        error!("relay task failed: {e}");
                    }
                    drop(semaphore_permit);
                });
            }
            Err(e) => error!("failed to accept connection: {e}"),
        }
    }
    Ok(())
}

pub async fn listen_tcp(args: &Cli) -> Result<()> {
    let host_listener = TcpListener::bind(&args.tcp_address)
        .await
        .context("failed to bind tcp listener")?;
    let conn_count_semaphore = Arc::new(Semaphore::new(args.max_concurrent_connections));
    let destination_address = parse_enclave_addr(&args.vsock_address)?;
    info!("Listening on tcp {}...", args.tcp_address);

    // Use semaphore to limit active connection count
    while let Ok(semaphore_permit) = conn_count_semaphore.clone().acquire_owned().await {
        match host_listener.accept().await {
            Ok((tcp_stream, _)) => {
                let buf_size = args.buffer_size;

                // Spawn new task to handle connection, task will now own semaphore
                // for the duration of the connection.
                tokio::spawn(async move {
                    let result = async {
                        let enclave_stream = connect_to_enclave(destination_address).await?;
                        let task = RelayTask::new(
                            ConnectionStream::TcpStreamType(tcp_stream),
                            ConnectionStream::EnclaveStreamType(enclave_stream),
                            buf_size,
                        )
                        .await?;
                        task.run().await
                    };
                    if let Err(e) = result.await {
                        error!("relay task failed: {e}");
                    }
                    drop(semaphore_permit);
                });
            }
            Err(e) => error!("failed to accept connection: {e}"),
        }
    }
    Ok(())
}

pub async fn listen_and_serve(args: &Cli) -> Result<()> {
    match args.tcp_to_vsock {
        true => listen_tcp(args).await,
        false => listen_vsock(args).await,
    }
}
