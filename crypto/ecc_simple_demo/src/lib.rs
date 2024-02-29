#[warn(non_snake_case)]
pub mod point;

use crate::point::{mod_exp, Point};
use rand::{thread_rng, Rng};
use std::hash::{DefaultHasher, Hash, Hasher};

/// 加解密的验证过程
/// y^2 = x^3+ax+b
/// 假设曲线为 y^2 = x^3 + x + 1 （mod 23）基点 G(0, 1)
/// mod_value和阶的关系；
pub struct ECC {
    G: Point,
    a: usize,
    b: usize,
    // 限定了x和y都只能在0～mod_value之间的取值范围
    mod_value: usize,
    n: usize,
}

const A: usize = 1;

impl ECC {
    pub fn new(g: Point, a: usize, b: usize, mod_value: usize, n: usize) -> Self {
        let G = g;
        Self {
            G,
            a,
            b,
            mod_value,
            n,
        }
    }

    pub fn point_addition(&self, p: Point, q: Point) -> Point {
        if p.eq(&Point { x: 0, y: 0 }) {
            return q;
        }
        if q.eq(&Point { x: 0, y: 0 }) {
            return p;
        }
        // 计算两点的斜率
        let m: usize;
        if p != q {
            m = ((q.y + self.mod_value - p.y)
                * mod_exp(
                    q.x + self.mod_value - p.x,
                    self.mod_value - 2,
                    self.mod_value,
                ))
                % self.mod_value;
        } else {
            m = ((3 * p.x.pow(2) + self.a) * mod_exp(2 * p.y, self.mod_value - 2, self.mod_value))
                % self.mod_value;
        }
        // println!("m: {:?}, p: {:?}, q: {:?}", m, p, q);
        let x_r;
        let y_r;
        if m != 0 {
            x_r = (m.pow(2) + self.mod_value * 2 - p.x - q.x) % self.mod_value;
            y_r = (m * (p.x + self.mod_value - x_r) + self.mod_value - p.y) % self.mod_value;
        } else {
            x_r = (2 * self.mod_value - p.x - q.x) % self.mod_value;
            y_r = (self.mod_value - p.y) % self.mod_value;
        }
        Point { x: x_r, y: y_r }
    }

    pub fn scalar_multiplication(&self, d: usize, point: Point) -> Point {
        let mut result = Point { x: 0, y: 0 };
        let mut current = point;
        let mut d = d;
        while d > 0 {
            if d % 2 == 1 {
                result = self.point_addition(result, current);
            }
            current = self.point_addition(current, current);
            d >>= 1;
        }
        result
    }

    pub fn generate_key_pair(&self) -> (usize, Point) {
        let random = thread_rng().gen_range(1..self.n);
        let point = self.scalar_multiplication(random, self.G);
        (random, point)
    }

    pub fn sign(&self, private_key: usize, msg: usize) -> (usize, usize, bool) {
        // 选择随机数k
        let k = thread_rng().gen_range(1..self.n);
        println!("sign k: {:?}", k);
        // R = k * G
        let r_point = self.scalar_multiplication(k, self.G);
        println!("r point: {:?}", r_point);
        let r = r_point.x;
        // s = (k^-1)*(m+rd) mod n
        let s = (mod_exp(k, self.n - 2, self.n) * (msg + r * private_key)) % self.n;
        (r, s, r_point.y % 2 == 0)
    }

    pub fn verify(&self, sig: (usize, usize, bool), msg: usize, public_key: Point) -> bool {
        // 计算 s^-1*(m*G+r*Q)
        let m_g = self.scalar_multiplication(msg, self.G);
        let r_q = self.scalar_multiplication(sig.0, public_key);
        let s_inv = mod_exp(sig.1, self.n - 2, self.n);
        let mg_sinv = self.scalar_multiplication(s_inv, m_g);
        let rq_sinv = self.scalar_multiplication(s_inv, r_q);
        let point = self.point_addition(mg_sinv, rq_sinv);
        println!("result: {:?}", point);
        point.x == sig.0
    }

    fn concat_point(&self, a: Point, b: Point) -> Point {
        Point {
            x: a.x + b.x % self.mod_value,
            y: a.y + b.y % self.mod_value,
        }
    }

    pub fn schnorr_sign(&self, private_key: usize, msg: Point) -> (Point, usize) {
        let k = thread_rng().gen_range(1..self.n);
        println!("schnorr sign k: {:?}", k);
        let r_point = self.scalar_multiplication(k, self.G);
        // 将两个点联结生成hash
        // let p = self.concat_point(msg, r_point);
        let p = msg;
        let mut hasher = DefaultHasher::new();
        let _ = &p.hash(&mut hasher);
        let hash_result = (hasher.finish() % (self.n as u64)) as usize;
        let sig = (k + hash_result * private_key) % self.n;
        (r_point, sig)
    }

    pub fn schnorr_sig_verify(&self, public_key: Point, sig: (Point, usize), msg: Point) -> bool {
        // sG = (k+h*d)*G = kG+h*Q = R+hQ
        let s_g = self.scalar_multiplication(sig.1, self.G);
        // R+Hash*Q
        // let p = self.concat_point(msg, sig.0);
        let p = msg;
        let mut hasher = DefaultHasher::new();
        let _ = &p.hash(&mut hasher);
        let hash_result = (hasher.finish() % (self.n as u64)) as usize;
        let hash_pub_key_mul = self.scalar_multiplication(hash_result, public_key);
        let r_point_add_hash_pub = self.point_addition(sig.0, hash_pub_key_mul);
        println!("sG: {:?}", s_g);
        println!(
            "R+hash*Q: {:?}, hash*Q: {:?}， hash: {hash_result}",
            r_point_add_hash_pub, hash_pub_key_mul
        );
        s_g.x == r_point_add_hash_pub.x && s_g.y == r_point_add_hash_pub.y
    }

    pub fn schnorr_mpc_key_shard_gen(&self, shard_num: usize) -> (Vec<usize>, Point) {
        let mut shards = Vec::new();
        let mut sum_key = 0;
        for i in 0..shard_num {
            let key = self.generate_key_pair();
            sum_key = (sum_key + key.0) % self.n;
            shards.push(key.0);
        }
        // 生成公共对外公钥
        let point = self.scalar_multiplication(sum_key, self.G);
        (shards, point)
    }

    // msg被编写到曲线上
    pub fn encrypt(&self, pub_key: Point, msg: Point) -> (Point, Point) {
        // 选取随机数k
        let k = thread_rng().gen_range(1..self.n);
        // enc = msg+k*Q
        let r_point = self.scalar_multiplication(k, self.G);
        let k_q = self.scalar_multiplication(k, pub_key);
        println!("kQ = {:?}", k_q);
        (r_point, self.point_addition(msg, k_q))
    }

    pub fn decrypt(&self, private_key: usize, cipher: (Point, Point)) -> Point {
        let addition = self.scalar_multiplication(private_key, cipher.0);
        println!("dR = {:?}", addition);
        let neg_addition = Point {
            y: ((addition.y as isize).wrapping_neg() % self.mod_value as isize
                + self.mod_value as isize) as usize,
            x: addition.x,
        };
        println!("-dR = {:?}", neg_addition);
        self.point_addition(cipher.1, neg_addition)
    }

    fn legendre_symbol(a: usize, p: usize) -> isize {
        // a^(1/2*(p-1)) mod p
        let result = mod_exp(a, (p - 1) / 2, p);
        if result == p - 1 {
            -1
        } else {
            result as isize
        }
    }

    fn tonelli_shanks(a: usize, p: usize) -> Option<usize> {
        if ECC::legendre_symbol(a, p) != 1 {
            // a 不是 p 的二次剩余
            return None;
        }

        let mut s = p - 1;
        let mut e = 0;

        while s % 2 == 0 {
            s /= 2;
            e += 1;
        }

        // 在 Tonelli-Shanks 算法中，当 e = 0时候表示p-1是2的幂，可以简化计算，直接计算a^((s+1)/2)的模p值
        if e == 0 {
            // 当 e = 0 时，直接计算解
            let x = mod_exp(a, (s + 1) / 2, p);
            return Some(x);
        }

        let mut n = 2;
        while ECC::legendre_symbol(n, p) != -1 {
            n += 1;
        }

        let mut x = mod_exp(a, (s + 1) / 2, p);
        let mut b = mod_exp(a, s, p);
        let mut g = mod_exp(n, s, p);

        for _ in 0..e - 1 {
            let mut temp = b;
            let mut r = 0;

            while temp != 1 {
                temp = (temp * temp) % p;
                r += 1;
            }

            // r等于0时候特殊处理
            if r == 0 {
                // 当 r = 0 时，直接计算解
                let m = 2usize.pow(e - 1);
                let t = mod_exp(g, m, p);
                x = (x * t) % p;
                return Some(x);
            }

            let m = 2usize.pow(e - r - 1);
            let t = mod_exp(g, m, p);

            let gsqr = mod_exp(g, 2, p);

            g = t;
            x = (x * t) % p;
            b = (b * gsqr) % p;
        }

        Some(x)
    }

    fn find_y(&self, x: usize) -> Option<(usize, usize)> {
        let y_square = (mod_exp(x, 3, self.mod_value) + self.a * x + self.b) % self.mod_value;
        if ECC::legendre_symbol(y_square, self.mod_value) != 1 {
            // 如果 Legendre 符号不为 1，表示无解
            return None;
        }

        match ECC::tonelli_shanks(y_square, self.mod_value) {
            Some(y_positive) => {
                let y_negative = self.mod_value - y_positive;
                println!("k point: positive_y = {y_positive}, negative_y = {y_negative}");
                Some((y_positive, y_negative))
            }
            None => None,
        }
    }

    pub fn recover_pub_key(&self, sig: (usize, usize, bool), msg: usize) -> Option<Point> {
        let r_inv = mod_exp(sig.0, self.n - 2, self.n);
        let hr_inv = msg * r_inv % self.n;
        let sr_inv = sig.1 * r_inv % self.n;
        let hr_point = self.scalar_multiplication(hr_inv, self.G);
        let neg_point = Point {
            x: hr_point.x,
            y: ((hr_point.y as isize).wrapping_neg() % self.mod_value as isize
                + self.mod_value as isize) as usize,
        };
        let mut pub_key_point = None;
        // 恢复随机生成的点，必然存在，不存在的话签名的生成过程存在问题
        if let Some((y_positive, y_negative)) = self.find_y(sig.0) {
            let y;
            y = if (sig.2 && y_positive % 2 == 0) || (!sig.2 && y_positive % 2 != 0) {
                y_positive
            } else {
                y_negative
            };
            let sr_point = self.scalar_multiplication(sr_inv, Point { x: sig.0, y });
            pub_key_point = Some(self.point_addition(sr_point, neg_point));
        }
        pub_key_point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn test_generate_point() {
        let expect_points = [
            Point { x: 0, y: 1 },
            Point { x: 6, y: 19 },
            Point { x: 3, y: 13 },
            Point { x: 13, y: 16 },
            Point { x: 18, y: 3 },
            Point { x: 7, y: 11 },
            Point { x: 11, y: 3 },
            Point { x: 5, y: 19 },
            Point { x: 19, y: 18 },
            Point { x: 12, y: 4 },
            Point { x: 1, y: 16 },
            Point { x: 17, y: 20 },
            Point { x: 9, y: 16 },
            Point { x: 4, y: 0 },
            Point { x: 9, y: 7 },
            Point { x: 17, y: 3 },
            Point { x: 1, y: 7 },
            Point { x: 12, y: 19 },
            Point { x: 19, y: 5 },
            Point { x: 5, y: 4 },
            Point { x: 11, y: 20 },
            Point { x: 7, y: 12 },
            Point { x: 18, y: 20 },
            Point { x: 13, y: 7 },
            Point { x: 3, y: 10 },
            Point { x: 6, y: 4 },
            Point { x: 0, y: 22 },
            Point { x: 12, y: 3 },
            Point { x: 0, y: 1 },
        ];
        let mod_val = 23;
        let n = 29;
        let ecc23 = ECC::new(Point { x: 0, y: 1 }, 1, 1, mod_val, n);
        let mut points = vec![];
        for i in 1..n + 1 {
            let point = ecc23.scalar_multiplication(i, ecc23.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        assert_eq!(points, expect_points);
    }

    #[test]
    fn verify_sig() {
        // 正常都是hash化之后映射到椭圆曲线的域的范围
        let msg = 88;
        let mod_val = 29;
        let n = 37;
        let mut points = vec![];
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        for i in 1..n + 1 + 1 {
            let point = ecc29.scalar_multiplication(i, ecc29.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let key = ecc29.generate_key_pair();
        let sig = ecc29.sign(key.0, msg);
        println!("key: {:?}, sig(r, s): {:?}", key, sig);
        let result = ecc29.verify(sig, msg, key.1);
        assert_eq!(result, true);
        // ecdsa签名的延展性
        // (mG+r*Q)/(n-s) = (mG+rdG)/(n-s) = ((mG+rdG)*G)/(n-s)G = ((mG+rdG)*G)/s*G
        // = ((mG+rdG)*G)/(k^-1*(m+dr)G) = k*G = R
        let result2 = ecc29.verify((sig.0, ecc29.n - sig.1, sig.2), msg, key.1);
        assert_eq!(result2, true);
    }

    #[test]
    fn test_schnorr_sign() {
        let msg_origin = 45;
        let mod_val = 29;
        let n = 37;
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        let key = ecc29.generate_key_pair();
        let msg_point = ecc29.scalar_multiplication(msg_origin, ecc29.G);
        let sig = ecc29.schnorr_sign(key.0, msg_point);
        println!("sig: {:?}", sig);
        let verify_result = ecc29.schnorr_sig_verify(key.1, sig, msg_point);
        assert_eq!(verify_result, true);
    }

    #[test]
    fn test_schnorr_sign_mpc() {
        let shard_num = 2;
        let msg_origin = 45;
        let mod_val = 29;
        let n = 37;
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        let mut points = vec![];
        for i in 1..n + 1 + 1 {
            let point = ecc29.scalar_multiplication(i, ecc29.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let (shard_keys, pub_key): (Vec<usize>, Point) = ecc29.schnorr_mpc_key_shard_gen(shard_num);
        println!("shard keys: {:?}, pub_key: {:?}", shard_keys, pub_key);
        let msg_point = ecc29.scalar_multiplication(msg_origin, ecc29.G);
        let mut sig = (Point { x: 0, y: 0 }, 0);
        for i in 0..shard_num {
            let shard_private_key = shard_keys.get(i).unwrap();
            let shard_sig = ecc29.schnorr_sign(*shard_private_key, msg_point);
            println!("shard sig: {:?}", shard_sig);
            let shard_result = ecc29.schnorr_sig_verify(
                ecc29.scalar_multiplication(*shard_private_key, ecc29.G),
                shard_sig,
                msg_point,
            );
            assert_eq!(shard_result, true);
            // 对签名数据进行同态运算
            sig.0 = ecc29.point_addition(sig.0, shard_sig.0);
            sig.1 = (sig.1 + shard_sig.1) % ecc29.n;
            println!("sig: {:?}", sig);
        }

        // 验证合并的签名是否符合预期的公钥
        let result = ecc29.schnorr_sig_verify(pub_key, sig, msg_point);
        assert_eq!(result, true);
    }

    #[test]
    fn encrypt_decrypt() {
        let msg = 3;
        let mod_val = 29;
        let n = 37;
        let mut points = vec![];
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        for i in 1..n + 1 + 1 {
            let point = ecc29.scalar_multiplication(i, ecc29.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let key = ecc29.generate_key_pair();
        println!("key: {:?}", key);
        // 正常是不会采用直接倍点的，因为求解原数是困难问题，这里只是为了演示方便
        let msg_point = ecc29.scalar_multiplication(msg, ecc29.G);
        let cipher = ecc29.encrypt(key.1, msg_point);
        println!("cipher: ({:?}, {:?})", cipher.0, cipher.1);
        let dec = ecc29.decrypt(key.0, cipher);
        println!("plaintext: {:?}", dec);
        assert_eq!(dec, msg_point);
    }

    #[test]
    fn same_r_recover_private_key() {
        // 正常都是hash化之后映射到椭圆曲线的域的范围
        let msg = 88;
        let mod_val = 29;
        let n = 37;
        let mut points = vec![];
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        for i in 1..n + 1 + 1 {
            let point = ecc29.scalar_multiplication(i, ecc29.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let key = ecc29.generate_key_pair();
        let sig = ecc29.sign(key.0, msg);
        let mut sig2;
        let msg2 = 66;
        // 模拟使用了相同的随机数
        loop {
            sig2 = ecc29.sign(key.0, msg2);
            // 因为测试的域比较小
            if sig2.0 == sig.0 && sig2.2 == sig.2 {
                break;
            }
        }
        println!("sig: {:?}, sig2: {:?}", sig, sig2);
        // s1-s2 = k^(-1)(m1-m2)+k^-1*dr-k^-1*dr
        let s1_s2 = (n + sig.1 - sig2.1) % n;
        let msg_sub = (n + msg - msg2) % n;
        let s1_s2_inv = mod_exp(s1_s2, n - 2, n);
        println!("(s1-s2)^-1 = {s1_s2_inv}, m1 - m2 = {msg_sub}");
        let k_compute = (msg_sub * s1_s2_inv) % n;
        // 代入签名运算中 s = k^(-1)(m+dr) => d = (k*s-m)*r^-1
        let r_inv = mod_exp(sig.0, n - 2, n);
        let msg_mod = msg / n;
        let ks_m_sub = (n * msg_mod + k_compute * sig.1 - msg) % n;
        let d_compute = (ks_m_sub * r_inv) % n;
        println!(
            "random k: {k_compute}, private_key_compute: {d_compute}, key: {:?}",
            key
        );
        assert_eq!(d_compute, key.0);
    }

    #[test]
    fn recover_pub_key_test() {
        let msg = 19;
        let mod_val = 29;
        let n = 37;
        let mut points = vec![];
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        for i in 1..n + 1 + 1 {
            let point = ecc29.scalar_multiplication(i, ecc29.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let key = ecc29.generate_key_pair();
        println!("key: {:?}", key);
        let sig = ecc29.sign(key.0, msg);
        println!("key: {:?}, sig(r, s): {:?}", key, sig);
        let pub_key_point = ecc29.recover_pub_key(sig, msg);
        println!("pub key point {:?}", pub_key_point);
        assert_eq!(Some(key.1), pub_key_point);
    }

    #[test]
    fn diffie_hellman_exchange_test() {
        // 初始化曲线参数
        let mod_val = 29;
        let n = 37;
        let ecc29 = ECC::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        // 模拟2方生成各自的密钥对 bob:a aG alice b bG
        let bob_key_pair = ecc29.generate_key_pair();
        let alice_key_pair = ecc29.generate_key_pair();
        // bob 和 alice分别 恢复出公共的公钥 ab*G
        let com_pub_key_bob = ecc29.scalar_multiplication(bob_key_pair.0, alice_key_pair.1);
        let com_pub_key_alice = ecc29.scalar_multiplication(alice_key_pair.0, bob_key_pair.1);
        assert_eq!(com_pub_key_bob, com_pub_key_alice);
        // 然后使用得到的共同密钥进行对称加解密，比如用AES算法
    }
}
