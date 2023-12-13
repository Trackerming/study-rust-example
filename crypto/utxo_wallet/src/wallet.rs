use bitcoin::address::{Address, NetworkUnchecked};
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Network, PublicKey};
use std::ops::Add;
use std::str::FromStr;

pub struct Wallet {
    derive_pub_key: Xpub,
    derive_priv_key: Xpriv,
    network: Network,
}

impl Wallet {
    pub fn new(xpriv_str: &str, xpub: &str, network: &str) -> Self {
        Wallet {
            derive_priv_key: Xpriv::from_str(xpriv_str).unwrap(),
            derive_pub_key: Xpub::from_str(xpub).unwrap(),
            network: Network::from_str(network).unwrap(),
        }
    }

    pub fn get_public_key(&self, path_str: &str) -> String {
        let path = DerivationPath::from_str(path_str).expect("path not valid.");
        // we need secp256k1 context for key derivation
        let mut buf: Vec<AlignedType> = Vec::new();
        buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
        let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();
        let derive_key = self
            .derive_pub_key
            .derive_pub(&secp, &path)
            .expect("derive key failed");
        return derive_key.public_key.to_string();
    }

    pub fn get_address(&self, path_str: &str) -> Address {
        let public_key_str = self.get_public_key(path_str);
        let pub_key = PublicKey::from_str(&public_key_str).expect("public key from str error.");
        Address::p2pkh(&pub_key, self.network)
    }
}

pub struct SignInfo {
    address: Address,
    path: String,
}

impl SignInfo {
    pub fn new(address: Address, path: String) -> Self {
        Self { address, path }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_address() {
        let derive_priv_key = "tprv8foYNoo89nJXximCx8r5e8ZQEjK7UGW6nfiRHuaC6brhFohfUgja127af12849Q8ajLre5H6dGedA2ySTq1kofexsf59HcNBn9h5byxZ8X9";
        let derive_pub_key = "tpubDCe9Vgbq8BvVPodubsSKHbRRTvaUPtpEirbt8T1hwSKhB4NXtnda4cTqQDEM5mQrow5YUX3hf3FM3G8eN3wtUUQELJPUo6eUzL3C9r7kgNb";
        let path = "m/1/0";
        let wallet = Wallet::new(derive_priv_key, derive_pub_key, "testnet");
        let address = wallet.get_address(path);
        println!("address: {:?}", address);
        assert_eq!(address.to_string(), "n1FZyhk4fDzuLHhLbK5svsvzHzwcZJJz63");
    }
}
