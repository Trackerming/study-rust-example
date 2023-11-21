use anyhow::{Context, Result};
use clap::{command, Parser};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Semaphore,
};
use tracing::{error, info};

use crate::enclave_agnostic::enclave::{
    get_listener_server, parse_enclave_addr, DEFAULT_DEST_ADDR,
};
use crate::relay_task::RelayTask;

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

pub async fn listen_and_serve(args: &Cli) -> Result<()> {
    let (host_listener, listen_address, dest_address) = match args.tcp_to_vsock {
        true => {
            let address = parse_enclave_addr(&args.tcp_address)?;
            let listener = get_listener_server(address).await?;
            (listener, address, parse_enclave_addr(&args.vsock_address)?)
        }
        false => {
            let address = parse_enclave_addr(&args.vsock_address)?;
            let listener = get_listener_server(address).await?;
            (listener, address, parse_enclave_addr(&args.tcp_address)?)
        }
    };
    let connection_count_semaphore = Arc::new(Semaphore::new(args.max_concurrent_connections));
    info!("Listening on tcp {}...", listen_address);

    while let Ok(semaphore_permit) = connection_count_semaphore.clone().acquire_owned().await {
        match host_listener.accept().await {
            Ok((tcp_stream, _)) => {
                let buf_size = args.buffer_size;
                tokio::spawn(async move {
                    let result = async {
                        let task = RelayTask::new(tcp_stream, dest_address, buf_size).await?;
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
