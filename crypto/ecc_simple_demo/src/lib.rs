pub mod point;

use crate::point::{mod_exp, Point};
use rand::{thread_rng, Rng};
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

    pub fn sign(&self, private_key: usize, msg: usize) -> (usize, usize) {
        // 选择随机数k
        let k = thread_rng().gen_range(1..self.n);
        println!("sign k: {:?}", k);
        // R = k * G
        let r_point = self.scalar_multiplication(k, self.G);
        let r = r_point.x;
        // s = (k^-1)*(m+rd) mod n
        let s = (mod_exp(k, self.n - 2, self.n) * (msg + r * private_key)) % self.n;
        (r, s)
    }

    pub fn verify(&self, sig: (usize, usize), msg: usize, public_key: Point) -> bool {
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
}
