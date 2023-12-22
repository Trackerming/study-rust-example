use tracing::info;

pub fn encrypt(plaintext: String, password: String) -> anyhow::Result<()> {
    info!("plaintext: {plaintext}");
    Ok(())
}

pub fn decrypt(cipher: String, password: String) -> anyhow::Result<()> {
    info!("cipher: {cipher}");
    Ok(())
}
