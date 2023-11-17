use crate::permutation_list::*;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

struct DESChar {
    // 用64字节长度的二进制字符串表示key，比如"0100101010001111101...00101"
    key: String,
    // 初始化置换之后的key的长度，为56
    init_permutation_key: Vec<u8>,
    // 16轮有16个子密钥
    sub_keys: Vec<Vec<u8>>,
}

// 根据给定表格进行置换操作
fn permutation_by_table(origin: &[u8], table: &[u8], src_len: usize, dest_len: usize) -> Vec<u8> {
    let mut result = vec![0; dest_len];
    assert_eq!(origin.len(), src_len);
    for (index, pos) in table.iter().enumerate() {
        let pos_value = (*pos - 1) as usize;
        assert!(pos_value < src_len);
        result[index] = origin[pos_value];
    }
    assert_eq!(result.len(), dest_len);
    return result;
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
        self.init_permutation_key = permutation_by_table(&key_chars, &INIT_KEY_PERMUTATION, 64, 56);
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
            self.sub_keys.push(permutation_by_table(
                &combined,
                &SUB_KEY_PERMUTATION,
                56,
                48,
            ));
        }
    }

    fn plaintext_init_permutation(&self, plaintext: &str) -> Vec<u8> {
        // 先不考虑分组，按照标准明文的64个bit进行计算
        let plaintext_chars: Vec<u8> = plaintext
            .chars()
            .map(|c| c.to_digit(2).unwrap() as u8)
            .collect();
        return permutation_by_table(&plaintext_chars, &INIT_PLAINTEXT_PERMUTATION, 64, 64);
    }

    // E变换
    fn extend_right(&self, right: &mut [u8]) -> Vec<u8> {
        // 每四位一组对1和4分别向左右扩展
        return permutation_by_table(&right, &EXTEND_R_PLAINTEXT_PERMUTATION, 32, 48);
    }

    // S盒子变换
    fn s_box_convert(&self, input: &[u8], s_box: &[u8; 64]) -> [u8; 32] {
        let mut result: [u8; 32] = [0; 32];
        for i in 0..8 {
            let current_input = &input[(i * 6)..((i + 1) * 6)];
            // 取出input的坐标，bit1和2代表行 b3～b6代表列
            let rows = current_input[0] << 1 | current_input[5];
            let cols = (current_input[1] << 3)
                | (current_input[2] << 2)
                | (current_input[3] << 1)
                | current_input[4];
            let index = (rows * 16 + cols) as usize;
            assert!(index < 64);
            let value = s_box[index];
            let rslt: Vec<u8> = format!("{:b}", value)
                .chars()
                .map(|char| char.to_digit(2).unwrap() as u8)
                .collect();
            result[(i * 4)..((i + 1) * 4)].clone_from_slice(&rslt);
        }
        return result;
    }

    // P盒子变换
    fn p_box_permutation(&self, input: &[u8]) -> Vec<u8> {
        return permutation_by_table(&input, &P_R_SUB_PLAINTEXT_PERMUTATION, 32, 32);
    }

    // 算法轮对R的处理，包括E扩展、key_i异或、S盒子变换、P盒置换、L(上一轮的R)异或
    fn round_handle_r_func(
        &self,
        left: &Vec<u8>,
        prev_right: &mut Vec<u8>,
        current_round: usize,
    ) -> Vec<u8> {
        let mut current_right: Vec<u8> = vec![];
        // 对32位的right进行E扩展变成48位
        let right = self.extend_right(prev_right);
        // 与当前轮的密钥进行异或
        let mut xor_with_key: Vec<u8> = right
            .iter()
            .zip(self.sub_keys[current_round].iter())
            .map(|(a, b)| (*a) ^ (*b))
            .collect();
        // S盒变换
        let s_result = self.s_box_convert(&mut xor_with_key, &S[current_round]);
        // P盒变换
        let mut p_result = self.p_box_permutation(&s_result);
        current_right = p_result
            .iter()
            .zip(left.iter())
            .map(|(a, b)| a ^ b)
            .collect();
        assert_eq!(current_right.len(), 32);
        return current_right;
    }

    fn round_handle(&self, init_perm_plaintext: &mut [u8; 64]) {
        let (left, right) = init_perm_plaintext.split_at_mut(32);
        let mut left_clone = Arc::new(RefCell::new(left.to_vec()));
        let mut right_clone = Arc::new(RefCell::new(right.to_vec()));
        // 经过16轮计算得到最终的L和R
        for i in 0..ROUND {
            let mut current_r = self.round_handle_r_func(
                left_clone.clone().borrow().deref(),
                right_clone.clone().borrow_mut().deref_mut(),
                i,
            );
            // 上一轮的R变为这一轮的L
            left_clone
                .clone()
                .borrow_mut()
                .clone_from_slice(left_clone.clone().borrow().as_slice());
            right_clone
                .clone()
                .borrow_mut()
                .clone_from_slice(current_r.as_slice());
        }
        // 合并L和R并进行最终的逆变换
    }
}

#[cfg(test)]
mod test {
    use crate::des_char_impl::DESChar;
    use crate::permutation_list::{EXTEND_R_PLAINTEXT_PERMUTATION, S};
    use std::cell::RefCell;
    use std::sync::Arc;

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
        for sub_key in des_char.sub_keys.iter() {
            println!("{:?}", sub_key);
        }
        let plaintext = "0000000100100011010001010110011110001001101010111100110111101111";
        let init_permutation_plaintext = des_char.plaintext_init_permutation(plaintext);
        println!("{:?}", init_permutation_plaintext);
        let mut array = [
            32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9, 10, 11, 12, 13, 12, 13, 14, 15, 16, 17, 16,
            17, 18, 19, 20, 21, 20, 21, 22, 23, 24, 25, 24, 25, 26, 27, 28, 29, 28, 29, 30, 31, 32,
            1,
        ];
        array.sort();

        println!("{:?}", array);
    }

    #[test]
    fn test_s_box() {
        let mut des_char = DESChar::new(
            "0001001100110100010101110111100110011011101111001101111111110001".to_string(),
        );
        let input = [
            0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1,
            0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0,
        ];
        let result = des_char.s_box_convert(&input, &S[0]);
        println!("{:?}", result);
        println!("{}", format!("{:b}", 12));
    }

    #[test]
    fn test_slice_copy() {
        let left = [12, 34, 56];
        let right = [78, 90, 23];
        let right2 = [45, 67, 89];
        let mut left_clone = Arc::new(RefCell::new(left.to_vec()));
        let mut right_clone = Arc::new(RefCell::new(right.to_vec()));
        println!("before replace left: {:?}", left_clone.clone().borrow());
        println!("before replace right: {:?}", right_clone.clone().borrow());
        left_clone
            .clone()
            .borrow_mut()
            .clone_from_slice(right_clone.clone().borrow().as_slice());
        right_clone
            .clone()
            .borrow_mut()
            .clone_from_slice(right2.as_slice());
        println!("after replace left: {:?}", left_clone.clone().borrow());
        println!("after replace right: {:?}", right_clone.clone().borrow());
    }
}
