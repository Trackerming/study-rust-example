use enc_dec::base58::decode;
use hmac::{Hmac, Mac};
use secp256k1::{PublicKey, SecretKey};
use sha2::Sha512;
use std::fmt::{Display, Error};

// https://www.btcstudy.org/2023/10/09/bip-32-extended-keys-diagram/
// 私钥派生方式的实现
// - 普通的子拓展密钥私钥
//     - 数据为 父公钥+索引号（字符串的拼接）
//     - key为 父密钥的链码
// - 强化的子拓展密钥私钥
//     - 数据为 父私钥+索引号
//     - key为 父密钥的链码
// - 普通的子拓展密钥公钥
//     - 数据为 父公钥+索引号
//     - key为 父密钥的链码
// 为什么私钥派生之后能对应公钥的派生
// 在派生普通的子拓展密钥时，投入HMAC函数中的输入是相同的，所以得到的结果也是相同的；使用的结果的前面32bytes的即为私钥，
//   加上父私钥就可以得到子私钥，加上父公钥（椭圆曲线点加法）从而产生子公钥

const VERSION_BYTES_MAINNET_PUBLIC: [u8; 4] = [0x04, 0x88, 0xB2, 0x1E];
const VERSION_BYTES_MAINNET_PRIVATE: [u8; 4] = [0x04, 0x88, 0xAD, 0xE4];

fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut hmac = Hmac::<Sha512>::new_from_slice(key).unwrap();
    hmac.update(data);
    let result = hmac.finalize().into_bytes();
    result.to_vec()
}

#[derive(Debug)]
pub struct HDPrivKey {
    network: [u8; 4],
    child_number: u32,
    depth: u8,
    parent_fingerprint: [u8; 4],
    chain_code: [u8; 32],
    private_key: SecretKey,
}

impl HDPrivKey {
    pub fn from_str(extend_key: &str) -> Result<Self, Error> {
        let data = decode(extend_key.to_string());
        if data.len() != 82 {
            return Err(Error::default());
        };
        let network: [u8; 4] = [data[0], data[1], data[2], data[3]];
        let depth = data[4];
        let parent_fingerprint = data[5..9]
            .try_into()
            .expect("9 - 5 == 4, which is the Fingerprint length");
        let child_number = u32::from_be_bytes(data[9..13].try_into().expect("4 byte slice")).into();
        let chain_code = data[13..45]
            .try_into()
            .expect("45 - 13 == 32, which is the ChainCode length");
        let private_key = secp256k1::SecretKey::from_slice(&data[46..78]).unwrap();
        Ok(HDPrivKey {
            network,
            chain_code,
            depth,
            private_key,
            child_number,
            parent_fingerprint,
        })
    }
}

#[derive(Debug)]
pub struct HDPubKey {
    network: [u8; 4], // 将 “xprv” 0488ade4 或者 “xpub” 0488b21e 放在开头，以表示其内容。前者表示私钥，后者表示公钥
    child_number: u32, // 从父密钥派生出本密钥所用的索引号
    depth: u8,        // 从主密钥到本密钥经过多少次派生
    parent_fingerprint: [u8; 4], // 父 公钥 的 hash160 哈希值的前 4 字节。这可以在日后帮助定位父密钥
    chain_code: [u8; 32], // 额外的 32 字节的秘密值。没有这个值就无法派生子密钥
    public_key: PublicKey, // 私钥（加上 0x00 作为前缀），或者公钥
}

impl HDPubKey {
    pub fn from_str(extend_key: &str) -> Result<Self, Error> {
        let data = decode(extend_key.to_string());
        if data.len() != 82 {
            return Err(Error::default());
        };
        Ok(HDPubKey {
            network: [data[0], data[1], data[2], data[3]],
            depth: data[4],
            parent_fingerprint: data[5..9]
                .try_into()
                .expect("9 - 5 == 4, which is the Fingerprint length"),
            child_number: u32::from_be_bytes(data[9..13].try_into().expect("4 byte slice")).into(),
            chain_code: data[13..45]
                .try_into()
                .expect("45 - 13 == 32, which is the ChainCode length"),
            public_key: secp256k1::PublicKey::from_slice(&data[45..78]).unwrap(),
        })
    }
}

#[cfg(test)]
mod test_hd_wallet_derive {
    use super::*;

    #[test]
    fn test_hmac_sha512() {
        let key = b"key";
        let data = b"Hello world";
        let hmac_result = hmac_sha512(key, data);
        println!(
            "HMAC-SHA512 result: {:?}, len: {:?}",
            hmac_result,
            hmac_result.len()
        );
        assert_eq!(
            hmac_result,
            vec![
                228, 219, 53, 97, 183, 128, 185, 126, 213, 119, 126, 42, 143, 27, 158, 130, 214,
                57, 128, 228, 9, 183, 169, 191, 209, 113, 239, 153, 216, 132, 227, 85, 210, 67,
                107, 195, 179, 26, 175, 251, 86, 46, 160, 61, 30, 24, 214, 73, 132, 26, 52, 157,
                80, 155, 224, 93, 137, 217, 158, 137, 71, 146, 216, 34
            ]
        );
    }

    #[test]
    fn test_hd_key_decode() {
        let ext_priv_key_str = "xprvA26kMT4FRp7itFwxcLnk8Qw8fnqSzbFQeuSqSuW1fbAn4EWgBCWAAJAhMWG1z3JtNtfUGPUpSdUGp9FicMhBYkW4ZzfDZkxfBBnPPKfw1om";
        let ext_pub_key_str = "xpub6F66kxb9GBg26k2RiNKkVYssDpfwQ3yG28NSFHudDvhkw2qpijpQi6VBCmxhjJQkoUB3iCfiFAfZftwZaiguX4Nng3ShmTmrUD9w9cpUFP9";
        let hd_priv_key = HDPrivKey::from_str(ext_priv_key_str).unwrap();
        println!("hd_private_key: {:?}", hd_priv_key);
        let hd_pub_key = HDPubKey::from_str(ext_pub_key_str).unwrap();
        println!("hd_public_key: {:?}", hd_pub_key);
        assert_eq!(
            hd_pub_key.parent_fingerprint,
            hd_priv_key.parent_fingerprint
        );
    }
}
