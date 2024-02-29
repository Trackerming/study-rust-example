/*
 * 判断是否是素数，因为这里是简单的演示不考虑实际的大质数，所以可以这样实现
*/
use rand::{thread_rng, Rng};
use crypto_util::base_compute::mod_exp;

/*
 * 只能用在较小域的范围
*/
pub fn is_prime(val: usize) -> bool {
    for i in 1..val {
        if val % i == 0 && i != 1 {
            return false;
        }
    }
    true
}

/*
 * 只能用在较小域的范围
*/
pub fn mod_opposite(val: usize, mod_val: usize) -> usize {
    // opposite * val  = 1+k*mod_val
    let mut i = 0;
    let mut opp_val;
    loop {
        let mul_val = i * mod_val + 1;
        if mul_val % val == 0 {
            opp_val = mul_val / val;
            break;
        }
        println!("i = {i}, mul_val = {mul_val}, val = {val}");
        i += 1;
    }
    opp_val
}

pub struct RSA {
    p: usize,
    q: usize,
    n: usize,
    φ_n: usize,
}

impl RSA {
    pub fn new(p: usize, q: usize) -> RSA {
        assert!(is_prime(p));
        assert!(is_prime(q));
        let n = p * q;
        // 欧拉函数
        let φ_n = (p - 1) * (q - 1);
        RSA { p, q, n, φ_n }
    }

    pub fn generate_key_pair(&self) -> (usize, usize) {
        // 随机生成一个(1, φ_n)并且与n互素的数e
        let mut e = 2;
        loop {
            // 不用随机数，用随机数可能造成演示的生成key比较缓慢，这里采用递增方式
            // e = thread_rng().gen_range(1..self.φ_n);
            if self.φ_n % e != 0 {
                break;
            }
            e += 1;
        }
        // 计算模反元素
        let d = mod_opposite(e, self.φ_n);
        (e, d)
    }

    pub fn encrypt(&self, public_key: usize, msg: usize) -> usize {
        mod_exp(msg, public_key, self.n)
    }

    pub fn decrypt(&self, private_key: usize, cipher: usize) -> usize {
        mod_exp(cipher, private_key, self.n)
    }

    pub fn sign(&self, private_key: usize, msg: usize) -> usize {
        mod_exp(msg, private_key, self.n)
    }

    pub fn verify(&self, public_key: usize, signature: usize, msg: usize) -> bool {
        let rec_msg = mod_exp(signature, public_key, self.n);
        println!("rec_msg: {:?}, msg: {:?}", rec_msg, msg);
        msg == rec_msg
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_prime() {
        let is_prime_1 = is_prime(4);
        let is_prime_2 = is_prime(7);
        let is_prime_3 = is_prime(9);
        assert_eq!(is_prime_1, false);
        assert_eq!(is_prime_2, true);
        assert_eq!(is_prime_3, false);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let rsa = RSA::new(17, 11);
        let key_pair = rsa.generate_key_pair();
        println!("key_pair: {:?}", key_pair);
        let msg = 78;
        let cipher = rsa.encrypt(key_pair.0, msg);
        let plaintext = rsa.decrypt(key_pair.1, cipher);
        println!("cipher: {}, plaintext: {}", cipher, plaintext);
        assert_eq!(plaintext, msg);
    }

    #[test]
    fn test_sign_verify() {
        let rsa = RSA::new(17, 11);
        let key_pair = rsa.generate_key_pair();
        println!("key_pair: {:?}", key_pair);
        let msg = 98;
        let signature = rsa.sign(key_pair.1, msg);
        let verify_result = rsa.verify(key_pair.0, signature, msg);
        assert_eq!(verify_result, true);
    }
}
