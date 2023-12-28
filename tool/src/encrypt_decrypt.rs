use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::{
    aes::KeySize::KeySize256,
    aes_gcm::AesGcm,
    digest::Digest,
    sha3::{Sha3, Sha3Mode},
};
use rand::prelude::*;
use std::iter::repeat;
use std::{fmt::Write, string::String};
use tracing::info;

use crate::util::{hex_string_2_array, u8_array_convert_string};

/// 使用hash算法将输入的密码转化为固定字节的key用于对数据进行加解密
/// 目前暂定hash256和AES-256 GCM
///
///

fn generate_key(password: String) -> Vec<u8> {
    let mut sh = Box::new(Sha3::new(Sha3Mode::Keccak256));
    sh.input(&password.as_bytes());
    let mut out = vec![0u8; sh.output_bytes()];
    sh.result(&mut out);
    out
}

pub fn encrypt(plaintext: String, password: String) -> anyhow::Result<()> {
    info!("plaintext: {plaintext}");
    let key = generate_key(password);
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; 12];
    rng.fill_bytes(&mut iv);
    let mut aad = [0u8; 16];
    rng.fill_bytes(&mut aad);
    let mut aes_gcm = AesGcm::new(KeySize256, &key[..], &iv, &aad);
    let plaintext_array = plaintext.as_bytes();
    let mut out: Vec<u8> = repeat(0)
        .take((plaintext_array.len() as u64).try_into().unwrap())
        .collect();
    let mut out_tag: Vec<u8> = repeat(0).take(16).collect();
    aes_gcm.encrypt(&plaintext_array, &mut out[..], &mut out_tag);
    let out_str: String = u8_array_convert_string(&out);
    let iv_str = u8_array_convert_string(&iv);
    let tag_str = u8_array_convert_string(&out_tag);
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
    let cipher_arr = hex_string_2_array(&cipher);
    let iv = hex_string_2_array(&iv);
    let tag = hex_string_2_array(&tag);
    let aad = hex_string_2_array(&aad);
    let mut aes_gcm = AesGcm::new(KeySize256, &key[..], &iv, &aad);
    let mut out: Vec<u8> = repeat(0).take(cipher_arr.len()).collect();
    let result = aes_gcm.decrypt(&cipher_arr, &mut out, &tag);
    let plaintext = String::from_utf8(out.clone()).unwrap();
    println!(
        "decrypt success: {:?}, result: {:?}, plaintext: {:?}",
        result, out, plaintext
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
