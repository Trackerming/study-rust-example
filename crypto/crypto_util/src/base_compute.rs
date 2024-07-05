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

// 拓展欧几里得算法：a*x+b*y=gcd(a,b)，返回的位x，y和gcd
pub fn extend_euclidean_algorithm(mut a: isize, mut b: isize) -> (isize, isize, isize) {
    let (mut x1, mut y1, mut x2, mut y2) = (1 as isize, 0 as isize, 0 as isize, 1 as isize);
    while b > 0 {
        let (q, r) = euclidean_division(a as usize, b as usize);
        (a, b) = (b, r as isize);
        (x1, x2) = (x2, x1 - q as isize * x2);
        (y1, y2) = (y2, y1 - q as isize * y2);
    }
    return (a, x1, y1);
}

// ax+by=gcd(a,b),ax+kn=1,构造kn为mod_val（n）的倍数，则ax=1 mod n，x就是a的逆元
pub fn mod_inverse_by_eea(a: isize, mod_value: isize) -> Option<isize> {
    let (gcd_val, mut x, _) = extend_euclidean_algorithm(a, mod_value);
    return if gcd_val == 1 {
        if x < 0 {
            x += mod_value;
        }
        Some(x)
    } else {
        None
    };
}

// 递归实现拓展欧几里得算法，ax+by=gcd(a, b)
pub fn ext_gcd(a: isize, b: isize) -> (isize, isize) {
    return if b == 0 {
        (1, 0)
    } else {
        let (mut x, mut y) = ext_gcd(b, a % b);
        (x, y) = (y, x - (a / b) * y);
        (x, y)
    };
}

// 递归求解逆元
pub fn mod_inverse_recursive(a: isize, n: isize) -> Option<isize> {
    return if gcd(a as usize, n as usize) == 1 {
        let mut w = ext_gcd(a, n).0;
        if w < 0 {
            w += n;
        }
        Some(w)
    } else {
        None
    };
}

// 求解最大公约数，欧几里德算法计算
pub fn gcd(mut a: usize, mut b: usize) -> usize {
    // 如果a比b小，则交换值，保证a始终不小于b
    // 即便不加这段，下面计算的过程还是会在第一次循环调整过来
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
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
        let gcd_val = gcd(a, b);
        assert_eq!(gcd_val, 3);
        let gcd_val = gcd(b, a);
        assert_eq!(gcd_val, 3);
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

    #[test]
    fn test_extend_euclidean_algorithm() {
        let result = extend_euclidean_algorithm(30, 24);
        println!("result: {:?}", result);
        assert_eq!(result, (6, 1, -1));
        let result = ext_gcd(30, 24);
        assert_eq!(result, (1, -1));
    }

    #[test]
    fn test_get_mod_inverse() {
        let val = mod_inverse_by_eea(7, 69);
        println!("{} * 7 mod 69 = 1", val.unwrap());
        assert_eq!(val, Some(10));
        let val = mod_inverse_recursive(7, 69);
        assert_eq!(val, Some(10));
        let val = mod_inverse_recursive(9, 23);
        println!("{} * 9 mod 23 = 1", val.unwrap());
        assert_eq!(val, Some(18));
    }
}
