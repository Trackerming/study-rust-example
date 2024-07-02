// 有一组数据，其中只有一个数出现了奇数次，其余的数都出现了偶数次，一次遍历且不添加多余的内存开销找出这个数；
pub fn find_odd_times_ele(array: Vec<i32>) -> i32 {
    let mut xor = 0;
    // 自己和自己异或等于0，0与自己异或还是得到本身
    let _ = array.iter().for_each(|ele| xor = xor ^ ele);
    xor
}

// 有一组数组，有两个数出现了奇数次，其余的都出现了偶数次，找出这两个数
pub fn find_odd_times_2_ele(array: Vec<i32>) -> (i32, i32) {
    // 先找出两个奇数次出现元素的异或结果，a^b
    let xor = find_odd_times_ele(array.clone());
    // 取异或结果中最右边的一个非0bit（为1的bit），两个数在这个bit位上一定是不同的
    let diff_bit = (!xor + 1) & xor;
    let mut xor1 = 0;
    array.iter().for_each(|ele| {
        // 这个不同bit位上的元素必然只有一个满足，所以最后剩余的就是这个bit位为1的出现奇数次的元素
        if (ele & diff_bit) == 0 {
            xor1 ^= ele;
        }
    });
    (xor1, xor ^ xor1)
}

#[cfg(test)]
mod test_bit_op {
    use super::*;

    #[test]
    fn test_find_one_odd_ele() {
        let array = vec![1, 2, 4, 2, 1];
        let ele = find_odd_times_ele(array);
        assert_eq!(ele, 4);
    }

    #[test]
    fn test_find_2_odd_ele() {
        let array = vec![1, 2, 4, 2, 1, 6];
        let ele = find_odd_times_2_ele(array);
        assert_eq!(ele, (4, 6));
    }
}
