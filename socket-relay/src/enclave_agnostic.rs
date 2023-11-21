#[cfg(not(feature = "mock-vsock"))]
pub(crate) mod enclave {
    use anyhow::{anyhow, Result};
    use std::net::Shutdown;
    use tokio::io::AsyncWriteExt;
    use tokio_vsock::{VsockAddr, VsockStream};

    pub type EnclaveStream = VsockStream;
    pub type EnclaveAddr = VsockAddr;

    pub const DEFAULT_DEST_ADDR: &str = "16:8443";

    pub fn parse_enclave_addr(address: &str) -> Result<EnclaveAddr> {
        let mut address_split = address.split(':');
        let cid = address_split
            .next()
            .ok_or(anyhow!("missing cid  from vsock addr: {address}"))?
            .parse()?;
        let port = address_split
            .next()
            .ok_or(anyhow!("missing port from vsock addr: {address}"))?
            .parse()?;
        Ok(VsockAddr::new(cid, port))
    }

    pub async fn connect_to_enclave(address: EnclaveAddr) -> Result<EnclaveStream> {
        Ok(VsockStream::connect(address.cid(), address.port()).await?)
    }

    pub async fn shutdown_enclave_stream(stream: &mut EnclaveStream) {
        stream.shutdown(Shutdown::Both).ok();
    }
}

#[cfg(feature = "mock-vsock")]
pub(crate) mod enclave {
    use anyhow::{Context, Result};
    use std::{net::SocketAddr, str::FromStr};
    use tokio::{io::AsyncWriteExt, net::TcpStream};

    pub type EnclaveStream = TcpStream;
    pub type EnclaveAddr = SocketAddr;

    pub const DEFAULT_DEST_ADDR: &str = "127.0.0.1:9443";

    pub fn parse_enclave_addr(address: &str) -> Result<EnclaveAddr> {
        Ok(SocketAddr::from_str(address).context("error parsing desination address")?)
    }

    pub async fn connect_to_enclave(address: EnclaveAddr) -> Result<EnclaveStream> {
        Ok(TcpStream::connect(address).await?)
    }

    pub async fn shutdown_enclave_stream(stream: &mut EnclaveStream) {
        stream.shutdown().await.ok();
    }
}
