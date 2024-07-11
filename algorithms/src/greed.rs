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
}
