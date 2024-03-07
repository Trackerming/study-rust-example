use crypto_util::point::Point;
use ecc_simple_demo::ECC;
use rand::{thread_rng, Rng};

/// Sigma 零知识证明:知道秘密s ，且与公开输入 Q 满足离散对数关系 Q =w*G
/// 1. 承诺：P选择随机数r，计算R = r*G，发送R
/// 2. 挑战：V发送随机数e
/// 3. 响应：P计算响应 z = r+e*w
/// 4. 验证：V验证 z*G 是否等于 R+e*Q
///
pub struct SigmaZK {
    pub zk_ecc: ECC,
    secret: usize,
    pub pub_point: Point,
}

impl SigmaZK {
    pub fn new(point: Point, a: usize, b: usize, mod_val: usize, n: usize) -> Self {
        SigmaZK {
            zk_ecc: ECC::new(point, a, b, mod_val, n),
            secret: 0,
            pub_point: Point { x: 0, y: 0 },
        }
    }

    pub fn generate_secret(&mut self) {
        self.secret = thread_rng().gen_range(1..self.zk_ecc.n);
        self.pub_point = self
            .zk_ecc
            .scalar_multiplication(self.secret, self.zk_ecc.G);
    }

    pub fn generate_proof(&self, e: usize) -> (usize, Point) {
        let r = thread_rng().gen_range(1..self.zk_ecc.n);
        let r_point = self.zk_ecc.scalar_multiplication(r, self.zk_ecc.G);
        let z = (r + self.secret * e) % self.zk_ecc.n;
        (z, r_point)
    }
}

#[cfg(test)]
mod sigma_zk_test {
    use super::*;

    #[test]
    fn verify_sigma_zk() {
        let mod_val = 29;
        let n = 37;
        let mut zk_sigma = SigmaZK::new(Point { x: 2, y: 6 }, 4, 20, mod_val, n);
        // 生成秘密
        zk_sigma.generate_secret();
        let pub_point = zk_sigma.pub_point;
        // 发送随机数挑战
        let e = thread_rng().gen_range(1..n);
        let proof = zk_sigma.generate_proof(e);
        let z_g = zk_sigma
            .zk_ecc
            .scalar_multiplication(proof.0, zk_sigma.zk_ecc.G);
        let e_q = zk_sigma.zk_ecc.scalar_multiplication(e, pub_point);
        let r_q_point = zk_sigma.zk_ecc.point_addition(proof.1, e_q);
        println!("z*G: {:?}", z_g);
        println!("R+e*Q: {:?}", r_q_point);
        assert_eq!(z_g, r_q_point);
    }
}
