use std::collections::HashMap;

pub fn two_sum(arr: Vec<u8>, target: u8) -> Option<[usize; 2]> {
    let mut hashmap = HashMap::new();
    for (i, &num) in arr.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = hashmap.get(&complement) {
            return Some([index, i]);
        }
        hashmap.insert(num, i);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, Some([0, 1]));
    }
}
