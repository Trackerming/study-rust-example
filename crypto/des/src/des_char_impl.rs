use crate::permutation_list::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

struct DESChar {
    // 用64字节长度的二进制字符串表示key，比如"0100101010001111101...00101"
    key: String,
    // 初始化置换之后的key的长度，为56
    init_permutation_key: Vec<u8>,
    // 16轮有16个子密钥
    sub_keys: Vec<Vec<u8>>,
}

impl DESChar {
    pub fn new(key: String) -> Self {
        DESChar {
            key,
            init_permutation_key: vec![],
            sub_keys: Vec::with_capacity(16),
        }
    }
    // 初始密钥置换
    fn init_permutation_key(&mut self) {
        let key_chars: Vec<u8> = self
            .key
            .chars()
            .map(|c| c.to_digit(2).unwrap() as u8)
            .collect();
        // 进行置换
        let mut result: [u8; 56] = [0; 56];
        for (index, pos) in INIT_KEY_PERMUTATION.iter().enumerate() {
            let pos_value = (*pos - 1) as usize;
            assert!(pos_value < key_chars.len());
            result[index] = key_chars[pos_value];
        }
        self.init_permutation_key = result.to_vec();
    }

    fn get_l_and_r_of_init_key(&mut self) -> (&mut [u8], &mut [u8]) {
        self.init_permutation_key.split_at_mut(28)
        //(&mut self.init_permutation_key[0..28], &mut self.init_permutation_key[28..])
    }
    fn sub_key_permutation(&mut self) {
        // 按照shift表进行移位
        let (left, right) = self.init_permutation_key.split_at_mut(28);
        let mut left_clone = Rc::new(RefCell::new(left));
        let mut right_clone = Rc::new(RefCell::new(right));
        for i in 0..ROUND {
            left_clone
                .clone()
                .borrow_mut()
                .rotate_left(KEY_SHIFT_SIZE[i] as usize);
            right_clone
                .clone()
                .borrow_mut()
                .rotate_left(KEY_SHIFT_SIZE[i] as usize);
            let combined: Vec<u8> = [
                left_clone.clone().borrow().to_vec(),
                right_clone.clone().borrow().to_vec(),
            ]
            .concat();
            let mut result: [u8; 48] = [0; 48];
            for (index, pos) in SUB_KEY_PERMUTATION.iter().enumerate() {
                let pos_value = (*pos - 1) as usize;
                assert!(pos_value < combined.len());
                result[index] = combined[pos_value];
            }
            self.sub_keys.push(result.to_vec());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::des_char_impl::DESChar;

    #[test]
    fn test_init_key_per() {
        let mut des_char = DESChar::new(
            "0001001100110100010101110111100110011011101111001101111111110001".to_string(),
        );
        des_char.init_permutation_key();
        println!(
            "{:?} len : {:?}",
            des_char.init_permutation_key,
            des_char.init_permutation_key.len()
        );
        des_char.sub_key_permutation();
        println!("{:?}", des_char.sub_keys);
    }
}
