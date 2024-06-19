use crate::read_u8;
use std::io::{Cursor, Read};

pub struct Signature {
    r: Vec<u8>,
    s: Vec<u8>,
}

impl Signature {
    pub fn encode_der(&self) -> Vec<u8> {
        let mut result = Vec::new();
        // SEQUENCE
        result.push(0x30);
        // 两个签名的长度和类型以及长度字段的长度
        let len = self.r.len() + self.s.len() + 4;
        // 编码长度
        // 长度超过127字节，使用长形式编码长度
        if len > 127 {
            let mut len_bytes = Vec::new();
            // 长度4个字节
            len_bytes.push((len >> 24) as u8);
            len_bytes.push((len >> 16) as u8);
            len_bytes.push((len >> 8) as u8);
            len_bytes.push(len as u8);
            // 指定长度为长形式
            result.push(0x81);
            result.extend_from_slice(len_bytes.as_slice());
        } else {
            result.push(len as u8);
        }
        // INTEGER
        result.push(0x02);
        result.push(self.r.len() as u8);
        result.extend_from_slice(self.r.as_slice());
        // INTEGER
        result.push(0x02);
        result.push(self.s.len() as u8);
        result.extend_from_slice(&self.s);
        result
    }

    pub fn decode_der(der_bytes: &[u8]) -> Option<Signature> {
        let mut cursor = Cursor::new(der_bytes);
        let mut r_len = 0;
        let mut s_len = 0;

        // 检查是否为SEQUENCE结构
        if read_u8(&mut cursor).unwrap() != 0x30 {
            return None;
        }
        let mut len = read_u8(&mut cursor).unwrap() as usize;
        if len > 127 {
            let len_bytes = read_u8(&mut cursor).unwrap() as usize - 128;
            for _ in 0..len_bytes {
                len <<= 8;
                len += read_u8(&mut cursor).unwrap() as usize;
            }
        }
        // 读取r
        if read_u8(&mut cursor).unwrap() != 0x02 {
            return None;
        }
        r_len = read_u8(&mut cursor).unwrap() as usize;
        let mut r = vec![0; r_len];
        cursor.read_exact(&mut r).ok()?;
        // 读取s
        if read_u8(&mut cursor).unwrap() != 0x02 {
            return None;
        }
        s_len = read_u8(&mut cursor).unwrap() as usize;
        let mut s = vec![0; s_len];
        cursor.read_exact(&mut s).ok()?;
        Some(Signature { r, s })
    }
}

#[cfg(test)]
mod der_tests {
    use super::*;
    use crate::hex_string_to_bytes;

    #[test]
    fn test_sig_der_encode_decode() {
        let r = vec![0x30, 0x42];
        let s = vec![0x60, 0x89];
        let signature = Signature {
            r: r.clone(),
            s: s.clone(),
        };
        let enc_der = signature.encode_der();
        println!("sig encode DER: {:?}", enc_der);
        assert_eq!(enc_der, vec![48, 8, 2, 2, 48, 66, 2, 2, 96, 137]);
        if let Some(sig) = Signature::decode_der(enc_der.as_slice()) {
            assert_eq!(sig.r, r);
            assert_eq!(sig.s, s);
        }
    }

    #[test]
    fn test_sig_btc_sig() {
        let sig = "304402203910bb05714f5eeda38ecf5d437bedede4c28584f0e4a8316158dbef0003d597022013d2319d346fc72c95b4d0a2c6a183e4896d74aadbf550288145af5bf24d8b0d01";
        let sig_bytes = hex_string_to_bytes(sig);
        println!("sig bytes len {:?}: {:?}", sig_bytes.len(), sig_bytes);
        let r = vec![
            57, 16, 187, 5, 113, 79, 94, 237, 163, 142, 207, 93, 67, 123, 237, 237, 228, 194, 133,
            132, 240, 228, 168, 49, 97, 88, 219, 239, 0, 3, 213, 151,
        ];
        let s = vec![
            19, 210, 49, 157, 52, 111, 199, 44, 149, 180, 208, 162, 198, 161, 131, 228, 137, 109,
            116, 170, 219, 245, 80, 40, 129, 69, 175, 91, 242, 77, 139, 13,
        ];
        let sig = Signature {
            r: r.clone(),
            s: s.clone(),
        };
        let enc_sig = sig.encode_der();
        // encode 的sig末尾多加了一个与签名本身无关的0x01字节
        assert_eq!(enc_sig, sig_bytes[..sig_bytes.len() - 1]);
        if let Some(sig) = Signature::decode_der(&sig_bytes) {
            println!("sig r: {:?}", sig.r);
            println!("sig s: {:?}", sig.s);
            assert_eq!(sig.r, r);
            assert_eq!(sig.s, s);
        }
    }
}
