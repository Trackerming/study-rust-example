type Array2D = Vec<Vec<i32>>;

/// 数组如图，从左到右升序，从上到下升序
/// [
///     [1, 4, 7, 11, 15],
///     [2, 5, 8, 12, 19],
///     [3, 6, 9, 16, 22],
///     [10, 13, 14, 17, 24],
///     [18, 21, 23, 26, 30]
/// ]
pub fn search(array: &Array2D, elem: i32) -> Option<(usize, usize)> {
    let cols = array[0].len();
    let rows = array.len();
    if cols == 0 || rows == 0 {
        return None;
    }
    // 行
    let mut r = 0;
    // 列
    let mut c = cols - 1;
    while r < rows && c >= 0 {
        let current_value = array.get(r)?.get(c)?;
        if *current_value == elem {
            return Some((r, c));
        } else if *current_value < elem {
            r += 1;
            c = cols - 1;
        } else {
            c -= 1;
        }
    }
    None
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_search() {
        let array: Array2D = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let index_7 = search(&array, 7);
        assert_eq!(index_7, Some((0, 2)));
        let index_5 = search(&array, 5);
        assert_eq!(index_5, Some((1, 1)));
        let index_20 = search(&array, 20);
        assert_eq!(index_20, None);
        let index_30 = search(&array, 30);
        assert_eq!(index_30, Some((4, 4)));
    }
}
