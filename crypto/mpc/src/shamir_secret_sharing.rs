use crypto_util::base_compute::{mod_exp, mod_inverse};
use rand::Rng;

#[derive(Debug)]
pub struct SSS {
    // 恢复share最少所需的数量
    threshold: usize,
    // 素数，域的模数
    prime: usize,
    // 多项式的次数
    n: usize,
    polynomial: Vec<usize>,
}

impl SSS {
    // 初始化生成多项式
    pub fn generate_polynomial(secret: usize, k: usize, n: usize, prime: usize) -> Self {
        let mut rng = rand::thread_rng();
        // 按照顺序保存系数
        let mut coefficients = vec![secret];
        coefficients.extend((0..n - 1).map(|_| rng.gen_range(1..prime)));
        SSS {
            threshold: k,
            prime,
            n,
            polynomial: coefficients,
        }
    }

    pub fn generate_shares(&self) -> Vec<(usize, usize)> {
        // x取值1到k
        (1..=self.threshold)
            .map(|i| {
                let share = self
                    .polynomial
                    .iter()
                    .enumerate()
                    // 多项式计算，a+b*x+...+k*x^(k-1)
                    .map(|(idx, coeff)| coeff * mod_exp(i, idx, self.prime))
                    .sum::<usize>()
                    % self.prime;
                (i, share)
            })
            .collect()
    }

    pub fn recover_secret(&self, shares: Vec<(usize, usize)>) -> usize {
        let mut coefficients = vec![0; shares.len()];
        // 迭代计算拉格朗日系数
        for i in 0..shares.len() {
            let mut numerator = 1;
            let mut denominator = 1;

            // 计算了拉格朗日插值的分子部分
            for j in 0..shares.len() {
                // 过滤掉当前份额的索引
                if i != j {
                    // 计算分子部分 除x_i之外的所有的x相乘
                    numerator = (numerator * shares[j].0) % self.prime;
                    // 计算分母部分，x_i - x_j
                    denominator =
                        (denominator * (shares[i].0 + self.prime - shares[j].0)) % self.prime;
                }
            }
            let lagrange_coeff =
                (shares[i].1 * numerator * mod_inverse(denominator, self.prime)) % self.prime;
            for j in 0..shares.len() {
                // 这里的取模运算原理？
                coefficients[j] = (coefficients[j]
                    + lagrange_coeff * mod_exp(shares[j].0, j, self.prime))
                    % self.prime;
            }
        }
        println!("coefficients: {:?}", coefficients);
        coefficients[0]
    }
}

#[cfg(test)]
mod test_sss_module {
    use super::*;

    #[test]
    fn test_sss_split_recover() {
        let sss = SSS::generate_polynomial(38, 3, 3, 257);
        let shares = sss.generate_shares();
        let re_sec = sss.recover_secret(shares.clone());
        println!("sss: {:?}\n, shares: {:?}\nre_sec = {re_sec}", sss, shares);
        assert_eq!(re_sec, 38);
    }
}
