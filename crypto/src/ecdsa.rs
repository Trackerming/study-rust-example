// pub(crate) use secp256k1::Error;
use alloy_primitives::{keccak256, Address};
use secp256k1::{rand, KeyPair, PublicKey, Secp256k1, SecretKey};

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
        let secp = Secp256k1::new();
        //let key_pair = KeyPair::new(&secp, &mut rand::thread_rng());
        // let sec_bytes = key_pair.secret_key().secret_bytes();
        // println!("key_pair: {:?}", sec_bytes);
        let sec_key = SecretKey::from_slice(&[
            183, 86, 23, 238, 203, 77, 47, 21, 213, 164, 57, 86, 231, 24, 21, 81, 128, 69, 239, 87,
            10, 151, 104, 235, 204, 124, 219, 13, 3, 159, 6, 190,
        ])
        .unwrap();
        let pub_key = PublicKey::from_secret_key(&secp, &sec_key);
        println!("pub_key: {:?}", pub_key);
        let addr = to_eth_address(pub_key);
        println!("addr {:?}", addr);
        let expect_addr = Address::from_str("ef48c4f9c5d9db7abc64426e992c8e7563826fc7").unwrap();
        assert_eq!(addr, expect_addr);
    }
}
