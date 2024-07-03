use crate::point::Point;

pub fn mod_exp(base: usize, exponent: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

/*
 * 计算模逆元素，模逆元素是指在模m 的情况下，某个整数a 的逆元素，即满足a⋅x≡1 (mod m)的整数x
 * 问题可以转化为线性同余方程 ax+my=1 然后采用拓展欧几里得算法进行计算；
 * 拓展欧几里得算法的基本思想是利用欧几里得算法的迭代过程中求出的两个整数s和t
 *   a*s+m*t = gcd(a,m)，当gcd(a,m)=1 （即a和m互素）时候，可以得到 a*s+m*t=1，于是s就是所需求解的模逆元素
*/
pub fn mod_inverse(a: usize, m: usize) -> usize {
    let mut mn = (m, a);
    // 初始时，我们希望得到的结果ax+my=m，因为这是最大公约数的一个倍数，为了使得第一次迭代后得到的结果就是m（m也是模数，取模运算后结果依然为1）
    // 将x的系数初始化为0，y的系数初始化为1 第一次迭代中ax+my=0+m=m%m=1
    let mut xy = (0, 1);
    while mn.1 != 0 {
        // 这一行执行扩展欧几里得算法的主要计算步骤；
        // (mn.0 / mn.1) * xy.1 计算了(m/a)*y的值
        // xy.0 - (mn.0 / mn.1) * xy.1 计算了 x-(m/a)*y的值
        // 最后将计算结果更新到xy元组中
        xy = (xy.1, (xy.0 + m - ((mn.0 / mn.1) * xy.1) % m) % m);
        // xy = (xy.1, xy.0  - ((mn.0 / mn.1) * xy.1));
        // 这一行更新了 mn 的值，将 a 更新为 m，将 m 更新为 a 除以 m 的余数
        mn = (mn.1, mn.0 % mn.1);
    }
    // 如果得到的计算结果为负数，取模运算调整为正数
    /*    while xy.0 < 0 {
        xy.0 += m as isize;
    }*/
    xy.0
}

pub fn chinese_remainder_theorem(residues: &[usize], modules: &[usize]) -> usize {
    assert_eq!(residues.len(), modules.len());
    // 计算模数组中所有的元素的积
    let modules_product: usize = modules.iter().product();
    let mut result: usize = 0;
    for (index, residue) in residues.iter().enumerate() {
        // 计算M_i
        let temp_m = modules_product / modules[index];
        let temp = residue * temp_m * mod_inverse(temp_m, modules[index]);
        result = (result + temp) % modules_product
    }
    result
}

pub fn tate_pairing(p1: &Point, p2: &Point, scalar: usize, mod_value: usize) -> usize {
    let result = p1.mul(scalar, mod_value);
    (result.x * p2.x + result.y * p2.y) % mod_value
}

// 求解最大公约数，欧几里德算法计算
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    a
}

// 欧几里得除法定义：对于整数 𝑎 和 𝑏（其中 𝑏≠0），存在唯一的整数对 (𝑞,𝑟)，使得 𝑎=𝑏𝑞+𝑟，其中 𝑞 是商, 𝑟 是余数，且 0≤𝑟<|𝑏|
pub fn euclidean_division(a: usize, b: usize) -> (usize, usize) {
    assert_ne!(b, 0);
    (a / b, a % b)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod test_base_compute_mod {
    use super::*;

    #[test]
    fn test_mod_inverse() {
        let a = 17;
        let m = 19;
        let mod_inv = mod_inverse(a, m);
        println!("{mod_inv}");
        assert_eq!(mod_inv, 9);
    }

    #[test]
    fn test_gcd() {
        let a = 10;
        let b = 30;
        let gcd_val = gcd(a, b);
        assert_eq!(gcd_val, 10);
        let a = 33;
        let b = 30;
        let gcd = gcd(a, b);
        assert_eq!(gcd, 3);
        let a = 33;
        let b = 2;
        let gcd = crate::base_compute::gcd(a, b);
        assert_eq!(gcd, 1);
    }

    #[test]
    fn test_lcm() {
        let a = 10;
        let b = 30;
        let lcm_val = lcm(a, b);
        assert_eq!(lcm_val, 30);
        let a = 33;
        let b = 30;
        let lcm_val = lcm(a, b);
        assert_eq!(lcm_val, 330);
        let a = 33;
        let b = 2;
        let lcm_val = lcm(a, b);
        assert_eq!(lcm_val, 66);
    }

    #[test]
    fn test_crt() {
        let residues = [2, 3, 1];
        let modules = [3, 4, 5];
        let crt = chinese_remainder_theorem(&residues, &modules);
        assert_eq!(crt, 11);
    }

    fn capture_panic<F: FnOnce() -> (usize, usize) + std::panic::UnwindSafe>(
        f: F,
    ) -> Option<String> {
        let result = std::panic::catch_unwind(f);
        match result {
            Ok(_) => None,
            Err(err) => {
                if let Some(s) = err.downcast_ref::<&str>() {
                    Some(s.to_string())
                } else if let Some(s) = err.downcast_ref::<String>() {
                    Some(s.clone())
                } else {
                    Some("unknown panic type".to_string())
                }
            }
        }
    }

    #[test]
    fn test_euclidean_division() {
        let result = euclidean_division(10, 3);
        assert_eq!(result, (3, 1));
        let result = capture_panic(|| euclidean_division(3, 0));
        // println!("result: {:?}", result);
        assert_eq!(
            Some("assertion `left != right` failed\n  left: 0\n right: 0".to_string()),
            result
        );
    }
}
