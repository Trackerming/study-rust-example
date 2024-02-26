pub fn fibonacci_list(n: u128) -> u128 {
    let mut fibs = [0, 1];
    for i in 2..n + 1 {
        let cache = fibs[0];
        fibs[0] = fibs[1];
        fibs[1] = fibs[0] + cache;
    }
    if n == 0 {
        return fibs[0];
    }
    fibs[1]
}

#[cfg(test)]
mod test_fibonacci_number_mod {
    use super::*;

    #[test]
    fn test_fib() {
        let fib0 = fibonacci_list(0);
        let fib1 = fibonacci_list(1);
        let fib2 = fibonacci_list(2);
        let fib9 = fibonacci_list(9);
        let fib39 = fibonacci_list(39);
        let fib99 = fibonacci_list(99);
        assert_eq!(fib0, 0);
        assert_eq!(fib1, 1);
        assert_eq!(fib2, 1);
        assert_eq!(fib9, 34);
        assert_eq!(fib39, 63245986);
        assert_eq!(fib99, 218922995834555169026);
    }
}
