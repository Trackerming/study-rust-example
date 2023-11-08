use std::ops::Deref;

/// 从数组中找出任意重复的元素，不添加额外的存储和排序等方式
/// 数组的特点，长度为N，则元素为小于N的数据
/// 从前到后遍历数组，将元素与对应index的数据调换位置，如果调换的时候对应index上的元素是相等的，说明重复的找到
pub fn find_repeat_of_array(array: &mut Vec<i32>) -> Option<i32> {
    if array.len() <= 1 {
        return None;
    }
    let iter = array.iter();
    for ele in iter.enumerate() {
        // 交换元素的位置
        if ele.0 != *ele.1 as usize {
            // 如果元素相等则说明是重复的元素
            if *ele.1 == *array.get(*ele.1 as usize).unwrap() {
                return Some(*ele.1);
            } else {
                unsafe {
                    let ptr = array.get(0)? as *const i32 as *mut i32;
                    std::ptr::swap(ptr.add(ele.0), ptr.add(*ele.1 as usize));
                }
            }
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_repeat_of_array() {
        let mut array = vec![2, 3, 1, 0, 2, 5, 3];
        // [1, 3, 2, 0, 2, 5, 3]
        // [1, 0, 2, 3, 2, 5, 3]
        // [1, 0, 2, 3, 2, 5, 3]
        // [1, 0, 2, 3, 2...
        let result = find_repeat_of_array(&mut array);
        println!("array {:?}", array);
        assert_eq!(result, Some(2));
        let mut array = vec![1, 0, 3, 2, 5, 4];
        let result = find_repeat_of_array(&mut array);
        println!("array {:?}", array);
        assert_eq!(result, None);
        let mut array = vec![0];
        let result = find_repeat_of_array(&mut array);
        assert_eq!(result, None);
    }
}
