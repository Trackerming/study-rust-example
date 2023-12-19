use std::str::FromStr;
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

pub fn get_pub_key(private_key: &str) -> String {
    let secp = Secp256k1::new();
    let sec_key = SecretKey::from_str(private_key).unwrap();
    let pub_key = PublicKey::from_secret_key(&secp, &sec_key);
    return pub_key.to_string();
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn to_address() {
        let secp = Secp256k1::new();
        // let key_pair = KeyPair::new(&secp, &mut rand::thread_rng());
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

    #[test]
    fn get_pub_key_test() {
        let private_key = "92d6ca31191fc4d404e82ac63dc21e2514dda7430a7e68f44a70946d710fcb62";
        let pub_key = get_pub_key(private_key);
        assert_eq!(
            pub_key,
            "03f3362af380ba3feb2d5b08452cd6274385a93471681f64466432d9779a25ff06"
        );
        println!("pub_key: {pub_key}");
    }
}
