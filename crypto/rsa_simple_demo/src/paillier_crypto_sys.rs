use crate::rsa::mod_opposite;
use crypto_util::base_compute::{gcd, lcm, mod_inverse};
use num_bigint::{BigUint, ToBigUint};
use num_traits::ToPrimitive;
use rand::Rng;

pub struct PaillierCryptoSys {
    p: usize,
    q: usize,
    n: usize,
    g_lambda: usize,
}

pub struct RSAKeyPair {
    private_key: (usize, usize),
    public_key: (usize, usize),
}

impl RSAKeyPair {
    pub fn new(n: usize, g: usize, g_lambda: usize, g_mu: usize) -> Self {
        RSAKeyPair {
            private_key: (g_lambda, g_mu),
            public_key: (n, g),
        }
    }
}

impl PaillierCryptoSys {
    // 为防止计算数值过大，这里最好限定p q的范围在100以内
    pub fn new(p: usize, q: usize) -> PaillierCryptoSys {
        // 初始化密钥相关参数
        PaillierCryptoSys {
            p,
            q,
            n: p * q,
            g_lambda: lcm(p - 1, q - 1),
        }
    }

    fn get_random(min_val: usize, max_val: usize, prime_for_val: usize) -> usize {
        let mut r = rand::thread_rng().gen_range(min_val..max_val);
        loop {
            // 判断随机数符合要求
            if gcd(r, prime_for_val) != 1 {
                r = rand::thread_rng().gen_range(0..max_val);
            } else {
                break;
            }
        }
        r
    }

    fn mod_pow(base: usize, exp: usize, modulus: usize) -> usize {
        let result =
            BigUint::from(base).modpow(&exp.to_biguint().unwrap(), &modulus.to_biguint().unwrap());
        result.to_usize().unwrap()
    }

    pub fn gen_key_pair(&self) -> RSAKeyPair {
        // 生成随机数g
        let n_2 = self.n.pow(2);
        let g = PaillierCryptoSys::get_random(0, n_2, n_2);
        // 这里实际情况要考虑大数的计算和优化，这里仅仅做原理上的演示
        let l_x = Self::mod_pow(g, self.g_lambda, n_2);
        let l = (l_x - 1) / self.n;
        // TODO: 替换为mod_inverse之后加解密的UT有一定的概率失败
        let g_mu = mod_opposite(l, self.n);
        println!(
            "g = {g}, l_x = {l_x}, l = {l}, g_mu = {g_mu}, n = {}",
            self.n
        );
        RSAKeyPair::new(self.n, g, self.g_lambda, g_mu)
    }

    pub fn encrypt(&self, public_key: (usize, usize), msg: usize) -> usize {
        let r = PaillierCryptoSys::get_random(0, self.n, self.n);
        let n_2 = self.n.pow(2);
        let k1 = Self::mod_pow(public_key.1, msg, n_2);
        let k2 = Self::mod_pow(r, self.n, n_2);
        // 得到密文
        let cipher = k1 * k2 % n_2;
        println!("encrypt cipher: {}", cipher);
        cipher
    }

    pub fn decrypt(&self, private_key: (usize, usize), cipher: usize) -> usize {
        let n_2 = self.n.pow(2);
        let l_x = Self::mod_pow(cipher, private_key.0, n_2);
        let l = (l_x - 1) / self.n;
        let plaintext = private_key.1 * l % self.n;
        println!("decrypt plaintext: {:?}", plaintext);
        plaintext
    }

    // 密文相加
    pub fn homomorphism_add(&self, cipher1: usize, cipher2: usize) -> usize {
        let n_2 = self.n.pow(2);
        (cipher1 * cipher2) % n_2
    }
}

#[cfg(test)]
mod test_paillier_crypto_sys {
    use super::*;

    #[test]
    fn test_encrypt_deccypt() {
        let paillier_sys = PaillierCryptoSys::new(17, 19);
        let key_pair = paillier_sys.gen_key_pair();
        let msg = 10;
        let cipher = paillier_sys.encrypt(key_pair.public_key, msg);
        println!("cipher: {:?}", cipher);
        let plaintext = paillier_sys.decrypt(key_pair.private_key, cipher);
        println!("plaintext: {:?}", plaintext);
        assert_eq!(plaintext, msg);
    }

    #[test]
    fn test_homomorphism() {
        let paillier_sys = PaillierCryptoSys::new(17, 19);
        let key_pair = paillier_sys.gen_key_pair();
        let msg1 = 10;
        let msg2 = 2;
        let cipher1 = paillier_sys.encrypt(key_pair.public_key, msg1);
        let cipher2 = paillier_sys.encrypt(key_pair.public_key, msg2);
        let total_cipher = paillier_sys.homomorphism_add(cipher1, cipher2);
        let add_plaintext = paillier_sys.decrypt(key_pair.private_key, total_cipher);
        println!("add plaintext: {:?}", add_plaintext);
        assert_eq!(add_plaintext, msg1 + msg2);
    }
}
