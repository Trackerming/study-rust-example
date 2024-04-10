use crate::ecc::ECC;
use crypto_util::point::Point;
use rand::{thread_rng, Rng};

// 如何在大数范围内计算并实现
// 无穷远点

pub struct Bls {
    pub ecc: ECC,
}

impl Bls {
    pub fn new(a: usize, b: usize, mod_value: usize, n: usize, g_point: Point) -> Self {
        let G = g_point;
        Self {
            ecc: ECC::new(G, a, b, mod_value, n),
        }
    }

    pub fn gen_key_pair(&self) -> (usize, Point) {
        let random = thread_rng().gen_range(1..self.ecc.n);
        let point = self.ecc.scalar_multiplication(random, self.ecc.G);
        (random, point)
    }

    fn mock_hash(message: &[u8]) -> usize {
        message.iter().map(|m| *m as usize).sum()
    }

    pub fn sign_message(&self, private_key: usize, message: &[u8]) -> Point {
        // 哈希：对消息 m 进行哈希处理，生成一个大整数 H = Hash(m)，其中 Hash() 是一个哈希函数，通常使用安全的哈希算法如 SHA-256
        // 这里模拟计算消息的hash得到一个数值
        let mock_hash: usize = Bls::mock_hash(message);
        println!("Hash(m) = {:?}", mock_hash);
        // 签名：计算签名点 s = x * H
        self.ecc
            .scalar_multiplication(mock_hash * private_key % self.ecc.n, self.ecc.G)
    }

    pub fn verify(&self, public_key: Point, message: &[u8], sig: Point) -> bool {
        let mock_hash = Bls::mock_hash(message);
        // s*G =? H*Q
        let h_q = self.ecc.scalar_multiplication(mock_hash, public_key);
        println!(
            "Hash(m) = {:?}, S = s*G = {:?}, Hash(m)*Q = {:?}",
            mock_hash, sig, h_q
        );
        sig == h_q
    }

    pub fn aggregate_public_key(&self, public_keys: Vec<Point>) -> Point {
        let mut point = Point { x: 0, y: 0 };
        for pub_key in public_keys {
            point = self.ecc.point_addition(pub_key, point);
        }
        point
    }

    pub fn batch_verify(
        &self,
        sigs: Vec<Point>,
        message: &[u8],
        aggregate_public_key: Point,
    ) -> bool {
        // 比起每个签名都验证一次，这里只需要对密文做聚合，即可验证所有的签名是否合法
        // H*(Q1+Q2)=H*(x1*G+x2*G)=H*(x1)*G+H*(x2)*G
        let mut agg_sig = Point { x: 0, y: 0 };
        for sig in sigs {
            agg_sig = self.ecc.point_addition(agg_sig, sig);
        }
        let mock_hash = Bls::mock_hash(message);
        let h_q_agg = self
            .ecc
            .scalar_multiplication(mock_hash, aggregate_public_key);
        println!(
            "Hash(m) = {:?}, Sum(S) = si*G = {:?}, Hash(m)*Qi = {:?}",
            mock_hash, agg_sig, h_q_agg
        );
        agg_sig == h_q_agg
    }
}

#[cfg(test)]
mod test_bls {
    use super::*;

    #[test]
    fn test_sig_verify() {
        let mod_val = 29;
        let n = 37;
        let mut points = vec![];
        let bls29 = Bls::new(4, 20, mod_val, n, Point { x: 2, y: 6 });
        for i in 1..n + 2 {
            let point = bls29.ecc.scalar_multiplication(i, bls29.ecc.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let key_pair = bls29.gen_key_pair();
        let message = b"hello bls";
        let sig = bls29.sign_message(key_pair.0, message);
        let verify_result = bls29.verify(key_pair.1, message, sig);
        println!("verify bls sig: {:?}", verify_result);
        assert!(verify_result);
    }

    #[test]
    fn test_batch() {
        let mod_val = 29;
        let n = 37;
        let mut points = vec![];
        let bls29 = Bls::new(4, 20, mod_val, n, Point { x: 2, y: 6 });
        for i in 1..n + 2 {
            let point = bls29.ecc.scalar_multiplication(i, bls29.ecc.G);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        let num_sig = 10;
        let mut key_pairs = vec![];
        let mut pub_keys = vec![];
        for i in 0..num_sig {
            let key_pair = bls29.gen_key_pair();
            pub_keys.push(key_pair.1);
            key_pairs.push(key_pair);
        }
        // 签名同一个消息
        let msg = b"hello bls aggregate";
        let mut sigs = vec![];
        for i in 0..num_sig {
            let sig = bls29.sign_message(key_pairs[i].0, msg);
            sigs.push(sig);
        }
        let agg_pub_key = bls29.aggregate_public_key(pub_keys);
        assert!(bls29.batch_verify(sigs, msg, agg_pub_key));
    }
}
