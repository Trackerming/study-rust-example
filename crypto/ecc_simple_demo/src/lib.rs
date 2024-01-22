#![feature(core_intrinsics)]

pub mod point;

use crate::point::{mod_exp, Point};
use rand::{thread_rng, Rng};
use std::intrinsics::sqrtf32;
use std::ops::Deref;

/// SEGMAX是啥？
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
            n: n,
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
        let result = mod_exp(a, (p - 1) / 2, p);
        if result == p - 1 {
            -1
        } else {
            result as isize
        }
    }

    fn tonelli_shanks(n: usize, p: usize) -> Option<usize> {
        if ECC::legendre_symbol(n, p) != 1 {
            // 如果 Legendre 符号不为 1，表示无解
            return None;
        }

        let mut q = p - 1;
        let mut s = 0;
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }

        let mut z = 2;
        while ECC::legendre_symbol(z, p) != -1 {
            z += 1;
        }

        let mut c = mod_exp(z, q, p);
        let mut r = mod_exp(n, (q + 1) / 2, p);
        let mut t = mod_exp(n, q, p);

        let mut m = s;
        while t != 1 {
            let mut i = 0;
            let mut e = 2;
            while mod_exp(t, e, p) != 1 {
                i += 1;
                e *= 2;
            }

            let b = mod_exp(c, 2usize.pow((m - i - 1) as u32), p);
            r = (r * b) % p;
            t = (t * b * b) % p;
            c = (b * b) % p;
            m = i;
        }

        Some(r)
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
}
