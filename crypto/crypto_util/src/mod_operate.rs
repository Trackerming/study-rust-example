use std::fmt::Debug;
// + - * / %
use std::ops::{Add, Div, Mul, Rem, Sub};

// 定义一个新的 trait Num，包含 Add, Sub, Mul, Div 这些 trait 的约束
trait Num:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + PartialEq
    + PartialOrd
    + Sized
    + Debug
    + Copy
{
}

// 为所有满足条件的类型实现 Num trait
impl<T> Num for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + PartialEq
        + Debug
        + Sized
        + Copy
{
}

pub trait ModOperate<T: Num> {
    fn mod_abs(a: T, m: T) -> T;
    // 求解模逆
    fn mod_inverse(a: T, m: T) -> Option<T>;
    // 求模幂
    fn mod_exp(base: T, exponent: T, m: T) -> T;
    // 欧几里得除法求商和余数
    fn euclidean_division(a: T, b: T) -> (T, T);
    // 扩展欧几里得算法，迭代法
    fn extend_euclidean_algorithm(a: T, m: T) -> (T, T, T);
    // 扩展欧几里得算法，递归法
    fn extend_euclidean_recursive(a: T, m: T) -> (T, T);
    // 求解两个数的最大公约数，采用欧几里得辗转相除法
    fn gcd(a: T, b: T) -> T;
    // 欧拉定理
    fn euler_theorem(a: T, m: T) -> T;
    // 欧拉函数
    fn euler_phi(a: T, m: T) -> T;
}

struct CryptoMod {}

impl<T> ModOperate<T> for CryptoMod
where
    T: Num,
    T: From<isize> + Into<isize>,
{
    fn mod_abs(a: T, m: T) -> T {
        assert!(m > 0.into());
        let mut result = a;
        if a < m && a >= 0.into() {
            result = a;
        } else if a < 0.into() {
            while result < 0.into() {
                result = result + m;
            }
        } else {
            result = a % m;
        }
        return result;
    }

    fn mod_inverse(a: T, m: T) -> Option<T> {
        let (gcd_val, x, _) = Self::extend_euclidean_algorithm(a, m);
        return if gcd_val == 1.into() {
            return Some(Self::mod_abs(x, m));
        } else {
            None
        };
    }

    fn mod_exp(base: T, exponent: T, m: T) -> T {
        if m == 1.into() {
            return 0.into();
        }
        let mut result = 1.into();
        let mut base = base % m;
        let mut exponent = exponent;
        while exponent > 0.into() {
            if exponent % (2.into()) == (1.into()) {
                result = (result * base) % m;
            }
            exponent = exponent / (2.into());
            base = (base * base) % m;
        }
        result
    }

    fn euclidean_division(a: T, b: T) -> (T, T) {
        assert_ne!(b, 0.into());
        (a / b, a % b)
    }

    fn extend_euclidean_algorithm(mut a: T, mut m: T) -> (T, T, T) {
        let (mut x1, mut y1, mut x2, mut y2) = (1.into(), 0.into(), 0.into(), 1.into());
        while m > 0.into() {
            let (q, r) = Self::euclidean_division(a, m);
            (a, m) = (m, r);
            (x1, x2) = (x2, x1 - q * x2);
            (y1, y2) = (y2, y1 - q * y2);
        }
        (a, x1, y1)
    }

    fn extend_euclidean_recursive(a: T, m: T) -> (T, T) {
        return if m == 0.into() {
            (1.into(), 0.into())
        } else {
            let (mut x, mut y) = Self::extend_euclidean_recursive(m, a % m);
            (x, y) = (y, x - (a / m) * y);
            (x, y)
        };
    }

    fn gcd(mut a: T, mut b: T) -> T {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        while b > 0.into() {
            let rem = a % b;
            a = b;
            b = rem;
        }
        a
    }

    // 统计m范围内与m互质的元素的个数
    fn euler_phi(a: T, m: T) -> T {
        let mut count = 0.into();
        for i in 1..(m.into() + 1) {
            if Self::gcd(m, i.into()) == 1.into() {
                count = count + 1.into();
            }
        }
        return count;
    }

    // 欧拉定理
    fn euler_theorem(a: T, m: T) -> T {
        assert_eq!(CryptoMod::gcd(m, a), 1.into());
        let phi_m = CryptoMod::euler_phi(a, m);
        let result = CryptoMod::mod_exp(a, phi_m, m);
        return result;
    }
}

#[cfg(test)]
mod mod_operate_test {
    use super::*;

    #[test]
    fn test_mod_abs() {
        let result1 = CryptoMod::mod_abs(-10, 23);
        let result2 = CryptoMod::mod_abs(103, 23);
        let result3 = CryptoMod::mod_abs(10, 23);
        assert_eq!(result1, 13);
        assert_eq!(result2, 11);
        assert_eq!(result3, 10);
    }

    #[test]
    fn test_euclidean_division() {
        let result1 = CryptoMod::euclidean_division(12, 35);
        let result2 = CryptoMod::euclidean_division(12, 6);
        assert_eq!(result1, (0, 12));
        assert_eq!(result2, (2, 0));
    }

    #[test]
    fn test_extend_euclidean_algorithm() {
        let (a, m) = (13, 27);
        let result1 = CryptoMod::extend_euclidean_algorithm(a, m);
        let result2 = CryptoMod::extend_euclidean_recursive(a, m);
        println!("{:?}", result1);
        println!("{:?}", result2);
        assert_eq!(result1.1, result2.0);
        assert_eq!(result1.2, result2.1);
    }

    #[test]
    fn test_gcd() {
        let (a1, b1) = (246, 389);
        let result1 = CryptoMod::gcd(a1, b1);
        let (a2, b2) = (24, 36);
        let result2 = CryptoMod::gcd(a2, b2);
        let (a3, b3) = (24, 48);
        let result3 = CryptoMod::gcd(a3, b3);
        let (gcd_val, _, _) = CryptoMod::extend_euclidean_algorithm(a3, b3);
        println!("{result1}-{result2}-{result3}");
        assert_eq!(result1, 1);
        assert_eq!(result2, 12);
        assert_eq!(result3, 24);
        assert_eq!(result3, gcd_val);
    }

    #[test]
    fn test_mod_inverse() {
        let result = CryptoMod::mod_inverse(7, 69);
        assert_eq!(result, Some(10));
        let result = CryptoMod::mod_inverse(9, 23);
        assert_eq!(result, Some(18));
        let result = CryptoMod::mod_inverse(12, 22);
        assert_eq!(result, None);
    }

    #[test]
    fn test_mod_exp() {
        let result = CryptoMod::mod_exp(7, 5, 13);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_euler_theorem() {
        let (a, m) = (7, 15);
        let phi_m = CryptoMod::euler_phi(a, m);
        println!("欧拉函数：phi({})={}=phi(3)*phi(5)=2*4", m, phi_m);
        assert_eq!(phi_m, 8);
        let result = CryptoMod::euler_theorem(a, m);
        assert_eq!(result, 1);
        println!("欧拉定理：{}^phi_m({})=1 mod {}", a, m, m);
    }
}
