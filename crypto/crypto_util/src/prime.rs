// 统计范围[2，n）内有多少个质数
fn count_prime_up_to(n: usize) -> usize {
    (2..n).filter(|x| is_prime(*x)).count()
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

// 确定性素性判断：[埃拉托斯特尼筛法](https://zh.wikipedia.org/wiki/%E5%9F%83%E6%8B%89%E6%89%98%E6%96%AF%E7%89%B9%E5%B0%BC%E7%AD%9B%E6%B3%95)
fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_primes = vec![true; n + 1];
    let mut primes = Vec::new();
    // 0和1不是素数
    is_primes[0] = false;
    is_primes[1] = false;
    // 基础原理
    /*    for p in 2..n {
        if is_primes[p] {
            primes.push(p);
            let mut multiple = p * p;
            while multiple <= n {
                is_primes[multiple] = false;
                multiple += p;
            }
        }
    }*/
    // 优化标记空间到sqrt(n)
    for p in 2..=((n as f64).sqrt() as usize) {
        if is_primes[p] {
            let mut multiple = p * p;
            while multiple <= n {
                is_primes[multiple] = false;
                multiple += p;
            }
        }
    }
    for p in 2..=n {
        if is_primes[p] {
            primes.push(p);
        }
    }
    primes
}

#[cfg(test)]
mod test_prime {
    use super::*;

    #[test]
    fn test_prime_theorem() {
        let n = 1_000_000;
        let prime_nums = count_prime_up_to(n);
        let estimate_prime = (n as f64) / (n as f64).ln();
        println!(
            "(0, {n}) primes nums: {prime_nums}, estimate prime nums {}",
            estimate_prime
        );
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        let primes = sieve_of_eratosthenes(50);
        assert_eq!(
            primes,
            vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
        );
    }
}
