use crate::ecc::ECC;
use crypto_util::point::Point;
use rand::{thread_rng, Rng};

pub struct Eddsa {
    ecc: ECC,
}

impl Eddsa {
    // 返回低位和高位
    fn get_lower_high_bit(d: usize) -> (usize, usize) {
        (d ^ 0x0f, d ^ 0xf0)
    }

    // 模拟e的计算过程
    fn sha256_mock(&self, r: &Point, q: &Point, msg: usize) -> usize {
        (r.x + q.x + r.y + q.y + msg) % self.ecc.mod_value
    }

    pub fn new(g: Point, a: usize, b: usize, mod_value: usize, n: usize) -> Self {
        Eddsa {
            ecc: ECC::new(g, a, b, mod_value, n),
        }
    }

    pub fn generate_key_pair(&self) -> (usize, Point) {
        // 生成群内合法的元素d，
        let random = thread_rng().gen_range(1..self.ecc.n);
        // 模拟获取元素d的低256bit和高256bit，sha512(d) = lower256bit+higher256bit
        // 由于这里的有限域设置得比较小，所以忽略hash算法，直接取低4bit和高4bit
        let (y, _) = Eddsa::get_lower_high_bit(random);
        (random, self.ecc.scalar_multiplication(y, self.ecc.G))
    }

    pub fn sign(&self, private_key: usize, msg: usize) -> (Point, usize) {
        let (y, high_bits) = Eddsa::get_lower_high_bit(private_key);
        // 伪随机数，k = sha256(high_bits, M)
        // 这里依然使用低位模拟
        let k = (high_bits ^ msg) % self.ecc.n;
        let r_point = self.ecc.scalar_multiplication(k, self.ecc.G);
        let public_point = self.ecc.scalar_multiplication(y, self.ecc.G);
        // e = sha256(R, Y, M)
        let e = self.sha256_mock(&r_point, &public_point, msg);
        // 计算签名，s = k+e*y
        (r_point, (k + e * y) % self.ecc.n)
    }

    pub fn verify_sig(&self, msg: usize, sig: (Point, usize), public_key_point: Point) -> bool {
        let e_verify = self.sha256_mock(&sig.0, &public_key_point, msg);
        // s*G
        let s_point = self.ecc.scalar_multiplication(sig.1, self.ecc.G);
        // R+e_verify*Q
        let verify_point = self.ecc.point_addition(
            sig.0,
            self.ecc.scalar_multiplication(e_verify, public_key_point),
        );
        println!(
            "verify_sig: s*G = {:?}, R+e_v*Q = {:?}",
            s_point, verify_point
        );
        return s_point == verify_point;
    }
}

#[cfg(test)]
mod eddsa_tests {
    use super::*;

    #[test]
    fn test_verify_sig() {
        let msg = 88;
        let mod_val = 251;
        let msg = 88;
        let mod_val = 29;
        let n = 37;
        let ecc29 = Eddsa::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        let key = ecc29.generate_key_pair();
        println!("key: {:?}", key);
        let sig = ecc29.sign(key.0, msg);
        let verify_result = ecc29.verify_sig(msg, sig, key.1);
        assert_eq!(verify_result, true);
    }
}
