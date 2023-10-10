// pub(crate) use secp256k1::Error;
use alloy_primitives::{keccak256, Address};
use secp256k1::PublicKey;

pub fn to_eth_address(public_key: PublicKey) -> Address {
    // hash256 32 bytes
    let hash = keccak256(&public_key.serialize_uncompressed()[1..]);
    println!("public key hash: {:?}", &hash);
    // 取后20byte
    Address::from_slice(&hash[12..])
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn to_address() {
        let pub_key = PublicKey::from_str("040a8a9f08b8d4f8d378fc7056f74d2b9d4c56b49f1f28c83972e54f2b1588d7e0c71ae6a5934cc3152f9e3ca71161d3b55c1b5ed5a70b57de0cebabf8f777be39").unwrap();
        let addr = to_eth_address(pub_key);
        println!("addr {:?}", addr);
        let expect_addr = Address::from_str("742d35Cc6634C0532925a3b844Bc454e4438f44e").unwrap();
        assert_eq!(addr, expect_addr);
    }
}
