// 给定n种硬币，第i种硬币的面值为 coins[i-1]，目标金额为amt，
// 每种硬币可以重复选取，问能够凑出目标金额的**最少**硬币数量。如果无法凑出目标金额，则返回-1

pub fn coin_change_greedy(coins: &mut [i32], mut amt: i32) -> i32 {
    // 让coins有序
    coins.sort();
    let mut index = coins.len() - 1;
    let mut count = 0;
    // 还没有选取完毕，就继续循环贪心选取
    while amt > 0 {
        // 找到小于且最接近amt的那个coin
        while index > 0 && coins[index] > amt {
            index -= 1;
        }
        // 选择coins[index]
        amt -= coins[index];
        count += 1;
    }
    if amt == 0 {
        count
    } else {
        -1
    }
}

// 最大切分乘积问题
// 给定一个正整数n，将其切分为至少两个正整数的和，求切分后所有整数的乘积最大是多少
// 2*(n-2)>=n;这说明大于等于4的整数都应该被切分
// 如果切分方案中包含4的因子，那么它就应该被继续切分。最终的切分方案只应出现1、2、3这三种因子
// 剩下的就是比较是拆分几个3和几个2的问题
pub fn max_product_cutting(n: i32) -> i32 {
    // 当 n <= 3 时，必须切分出一个 1
    if n <= 3 {
        return 1 * (n - 1);
    }
    // 贪心地切分出 3 ，a 为 3 的个数，b 为余数
    let a = n / 3;
    let b = n % 3;
    if b == 1 {
        // 当余数为 1 时，将一对 1 * 3 （1+3=4）转化为 2 * 2
        3_i32.pow(a as u32 - 1) * 2 * 2
    } else if b == 2 {
        // 余数为2的适合，所以不做处理
        3_i32.pow(a as u32) * 2
    } else {
        // 当余数为0的适合，不做处理，此时已经是最大的因子3自己运算了
        3_i32.pow(a as u32)
    }
}

#[cfg(test)]
mod greedy_test {
    use super::*;

    #[test]
    fn test_coin_change_greedy() {
        let mut coins = [1, 5, 10, 20, 50, 100];
        let count = coin_change_greedy(&mut coins, 138);
        assert_eq!(count, 7);
        // 贪心不适用的特例
        let mut coins = [1, 20, 50];
        // 动态规划可以找到20 + 20 + 20
        let count = coin_change_greedy(&mut coins, 60);
        assert_eq!(count, 11);
    }

    #[test]
    fn test_max_product_cutting() {
        let max_product = max_product_cutting(3);
        assert_eq!(max_product, 2);
        let max_product = max_product_cutting(6);
        assert_eq!(max_product, 9);
        // 3*2*2>3*3*1
        let max_product = max_product_cutting(7);
        assert_eq!(max_product, 12);
        // 3*3*2
        let max_product = max_product_cutting(8);
        assert_eq!(max_product, 18);
    }
}
