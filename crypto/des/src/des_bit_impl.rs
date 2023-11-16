use crate::permutation_list::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;

struct DES {
    // 实际起作用的是56个bit，每个字节的最后一位用于奇偶校验，可以忽略，经过密钥置换之后会去掉奇偶校验位
    key: [u8; 8],
    // 经过初始置换的key
    init_permutation: RefCell<[u8; 7]>,
    // 加解密进行的round次数，DES为16次
    round: u8,
    // 用于快速得到对应bit的十进制值
    bit_map: HashMap<u8, u8>,
}

impl DES {
    pub fn new(key: [u8; 8], round: u8) -> Self {
        DES {
            key,
            round,
            init_permutation: RefCell::new([0; 7]),
            bit_map: HashMap::from([
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 32),
                (6, 64),
                (7, 128),
            ]),
        }
    }

    fn get_index(&self, pos: u8) -> (u8, u8) {
        // 按照字节表示的话，高位在左边，低位在右边，如果算出来的bit位为0，则在数值上为bit7
        (pos / 8, (7 - (pos % 8)))
    }

    /**
     * 这里是高位还是低位的排列
     */
    fn get_pos_value(&self, origin: &[u8], pos: u8) -> u8 {
        let (array_index, bit_index) = self.get_index(pos);
        assert_eq!(array_index < origin.len() as u8, true);
        let value = origin[array_index as usize];
        return (value & *self.bit_map.get(&bit_index).unwrap()) >> bit_index;
    }

    fn to_value(&self, array: &mut [u8], pos: u8, origin_pos_value: u8) {
        let (array_index, bit_index) = self.get_index(pos);
        let mut value = array[array_index as usize];
        let mut or_value: u8 = 0;
        if origin_pos_value == 1 {
            or_value = *self.bit_map.get(&bit_index).unwrap();
        }
        array[array_index as usize] = value | or_value;
    }

    fn replace_bit_by_list(&self, origin: &[u8], table: &[u8], result: &mut [u8]) {
        // 遍历置换表
        for (index, pos) in table.iter().enumerate() {
            // 获取当前表值所对应的位置的值
            let value_at_pos = self.get_pos_value(&origin[..], *pos);
            // 反转到当前表值的坐标的位置
            self.to_value(&mut result[..], index as u8, value_at_pos);
        }
    }

    pub fn init_key_permutation(&mut self) {
        self.replace_bit_by_list(
            &self.key,
            &INIT_KEY_PERMUTATION,
            self.init_permutation.borrow_mut().deref_mut(),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::DerefMut;

    #[test]
    fn list_by_order() {
        let mut array = SUB_KEY_PERMUTATION.to_vec();
        array.sort();
        println!("array: {:?}", array);
    }

    #[test]
    fn test_replace_bit_by_list() {
        let keys: [u8; 8] = [
            0b10101010, 0b01010101, 0b10101010, 0b01010101, 0b10101010, 0b01010101, 0b10101010,
            0b01010101,
        ];
        let mut des = DES::new(keys, 16);
        des.replace_bit_by_list(
            &des.key,
            &INIT_KEY_PERMUTATION[..],
            des.init_permutation.borrow_mut().deref_mut(),
        );
        println!("result: {:?}", des.init_permutation.borrow());
    }
}
