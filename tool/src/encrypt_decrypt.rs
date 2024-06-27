use aes_gcm::aead::{AeadMut, Buffer};
use aes_gcm::{
    aead::{Aead, AeadCore, OsRng},
    Aes256Gcm, AesGcm, Key, KeyInit, Nonce,
};
use rand::prelude::*;
use sha3::{Digest, Keccak256};
use std::iter::repeat;
use std::{fmt::Write, string::String};
use tracing::info;

use crate::util::{hex_string_2_array, u8_array_convert_string};

/// 使用hash算法将输入的密码转化为固定字节的key用于对数据进行加解密
/// 目前暂定hash256和AES-256 GCM
///
///

fn generate_key(password: String) -> Vec<u8> {
    let mut sh = Keccak256::new();
    sh.update(&password.as_bytes());
    let out = sh.finalize().to_vec();
    out
}

pub fn encrypt(plaintext: String, password: String) -> anyhow::Result<()> {
    info!("plaintext: {plaintext}");
    let key = generate_key(password);
    let mut rng = rand::thread_rng();
    let mut aad = [0u8; 16];
    rng.fill_bytes(&mut aad);
    let aes_key = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let mut aes_gcm = Aes256Gcm::new(&aes_key);
    let plaintext_array = plaintext.as_bytes();
    let nonce = Aes256Gcm::generate_nonce(OsRng);
    let mut out = aes_gcm
        .encrypt(
            &nonce,
            aes_gcm::aead::Payload {
                msg: plaintext_array,
                aad: aad.as_slice(),
            },
        )
        .unwrap();
    println!("out: {:?}", out);
    let index = out.len() - 16;
    let out_str: String = u8_array_convert_string(&out[..index]);
    let iv_str = u8_array_convert_string(&nonce.as_slice());
    let tag_str = u8_array_convert_string(&out[index..]);
    let aad_str = u8_array_convert_string(&aad);
    info!(
        "cipher: {:?}\n{:?}, \niv: {:?}\n tag:{:?}, aav: {:?}",
        out, out_str, iv_str, tag_str, aad_str
    );
    Ok(())
}

pub fn decrypt(
    cipher: String,
    password: String,
    iv: String,
    tag: String,
    aad: String,
) -> anyhow::Result<()> {
    info!("cipher: {cipher}");
    let key = generate_key(password);
    let mut cipher_arr = hex_string_2_array(&cipher);
    let mut nonce_arr = hex_string_2_array(&iv);
    let tag = hex_string_2_array(&tag);
    let aad = hex_string_2_array(&aad);
    let aes_key = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let mut aes_gcm = Aes256Gcm::new(&aes_key);
    cipher_arr.extend_from_slice(&tag);
    let result = aes_gcm
        .decrypt(
            &Nonce::from_slice(nonce_arr.as_slice()),
            aes_gcm::aead::Payload {
                msg: cipher_arr.as_slice(),
                aad: &aad,
            },
        )
        .unwrap();
    //let result = aes_gcm.decrypt(&cipher_arr, &mut out, &tag);
    let plaintext = String::from_utf8(result.clone()).unwrap();
    println!(
        "decrypt success, result: {:?}, plaintext: {:?}",
        result, plaintext
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_u8_array() {
        let s = "a964b90e21e229e510";
        let arr = [169, 100, 185, 14, 33, 226, 41, 229, 16];
        let arr_result = hex_string_2_array(s);
        let str_result = u8_array_convert_string(&arr);
        assert_eq!(arr.to_vec(), arr_result);
        assert_eq!(str_result, s.to_string());
    }
}
