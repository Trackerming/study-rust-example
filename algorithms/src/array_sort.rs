/// 思路：相邻的元素比较，每一轮将大的元素往后放，每轮就确定了本轮的最大值
/// 重点：在内层的循环中的变量控制
pub fn bubble_sort<T>(array: &mut [T])
where
    T: PartialOrd,
{
    let mut swapped;
    for mut i in 0..array.len() {
        swapped = false;
        for mut j in 0..(array.len() - i - 1) {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let len = arr.len();
    // 分区取半
    let pivot_index = len >> 1;
    // 基准元素放与末尾
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        // 小于等于基准元素的，往数组的左边扔
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    // 当前0～i-1的位置都是小于基准元素的，把基准元素放到i的位置，则左边比基准元素小，右边比基准元素大，
    arr.swap(i, len - 1);
    i
}

/// 快速排序思路：先分区，确定基准元素，然后递归的进行分区排序
/// 分区的思路：随机一个基准元素之后左边放比基准元素小的分段，右边放比基准元素大的分段
pub fn quick_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len <= 1 {
        return;
    }
    let pivot_index = partition(array);
    quick_sort(&mut array[0..pivot_index]);
    quick_sort(&mut array[pivot_index..len]);
}

/// 归并思路：拆分为两个更小的数组，
/// 第一次进入left：[12, 43, 7, 6] right: [9, 76, 98, 57]
/// 再次进入是left
///     [12, 43] 和 [7, 6]
/// 再次进入
///     [12] 和 [43]
/// 开始归并[12, 43]
/// 开始进入并 merge[7, 6]
/// 开始回到merge[12, 43] 和 [6, 7]
/// 得到left为[6, 7, 12, 43]
/// 开始类似的过程执行 right: [9, 76, 98, 57]
/// 最后归并最初的栈[6, 7, 12, 43]和[9, 57, 76, 98]
pub fn merge_sort<T>(array: &mut [T])
where
    T: PartialOrd + Clone,
{
    let len = array.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let mut left = array[0..mid].to_vec();
    let mut right = array[mid..len].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    let (mut i, mut j, mut k) = (0, 0, 0);
    // 对得到的左右数组进行归并排序
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            array[k] = left[i].clone();
            i += 1;
        } else {
            array[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        array[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        array[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble() {
        let mut array1 = [12, 43, 7, 6, 9, 76, 98, 57];
        bubble_sort(&mut array1);
        assert_eq!(array1, [6, 7, 9, 12, 43, 57, 76, 98]);
        let mut array2 = ["test", "str", "rust", "down", "study"];
        bubble_sort(&mut array2);
        assert_eq!(array2, ["down", "rust", "str", "study", "test"]);
    }

    #[test]
    fn test_quick() {
        let mut array1 = [12, 43, 7, 6, 9, 76, 98, 57];
        quick_sort(&mut array1);
        assert_eq!(array1, [6, 7, 9, 12, 43, 57, 76, 98]);
        let mut array2 = ["test", "str", "rust", "down", "study"];
        quick_sort(&mut array2);
        assert_eq!(array2, ["down", "rust", "str", "study", "test"]);
    }

    #[test]
    fn test_merge() {
        let mut array1 = [12, 43, 7, 6, 9, 76, 98, 57];
        merge_sort(&mut array1);
        assert_eq!(array1, [6, 7, 9, 12, 43, 57, 76, 98]);
        let mut array2 = ["test", "str", "rust", "down", "study"];
        merge_sort(&mut array2);
        assert_eq!(array2, ["down", "rust", "str", "study", "test"]);
    }
}
