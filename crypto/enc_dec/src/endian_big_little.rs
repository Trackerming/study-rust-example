// https://learnmeabitcoin.com/technical/general/little-endian/

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};

pub trait VecU8 {
    fn to_u8_vec(&self, max_value: u32, is_big_endian: bool) -> Vec<u8>;
}

pub trait Endian {
    fn to_big_endian(&self) -> String;
    fn to_little_endian(&self) -> String;
}

use std::fmt::Write;

pub trait HexStr {
    fn to_hex_string(&self) -> String;
}

impl HexStr for Vec<u8> {
    fn to_hex_string(&self) -> String {
        let mut result = String::new();
        for a in self.iter() {
            write!(result, "{:02x}", a).unwrap();
        }
        result
    }
}

impl VecU8 for BigUint {
    fn to_u8_vec(&self, max_value: u32, is_big_endian: bool) -> Vec<u8> {
        let mut result = vec![];
        let radix = BigUint::from(max_value);
        let (mut quotient, mut remainder) = self.div_rem(&radix);
        result.push(remainder.to_u8().unwrap());
        loop {
            let (quot, mut remainder) = quotient.div_rem(&radix);
            if quotient.is_zero() {
                break;
            }
            result.push(remainder.to_u8().unwrap());
            quotient = quot;
        }
        if is_big_endian {
            result.reverse();
        }
        result
    }
}

// BigUint有方法转换为BigEndian和LittleEndian，这里仅仅做演示实现
impl Endian for BigUint {
    fn to_big_endian(&self) -> String {
        let big_endian_u8_bytes = self.to_u8_vec(256, true);
        big_endian_u8_bytes.to_hex_string()
    }
    fn to_little_endian(&self) -> String {
        let little_endian_u8_bytes = self.to_u8_vec(256, false);
        little_endian_u8_bytes.to_hex_string()
    }
}

#[cfg(test)]
mod test_endian_module {
    use super::*;
    use num_traits::Num;

    #[test]
    fn test_vec_trait() {
        let test_int = BigUint::from_str_radix("1001283637283949347689273343", 10).unwrap();
        let result = test_int.to_u8_vec(256, false);
        assert_eq!(
            result,
            vec![255, 23, 237, 138, 92, 206, 169, 189, 14, 62, 60, 3]
        );
        println!("result: {:?}", result);
        let result = test_int.to_u8_vec(256, true);
        assert_eq!(
            result,
            vec![3, 60, 62, 14, 189, 169, 206, 92, 138, 237, 23, 255]
        );
        println!("result: {:?}", result);
    }

    #[test]
    fn test_to_big_endian() {
        let test_int = BigUint::from_str_radix("1001283637283949347689273343", 10).unwrap();
        let big_endian = test_int.to_big_endian();
        println!("big_endian: {big_endian}");
        assert_eq!(big_endian, "033c3e0ebda9ce5c8aed17ff");
    }

    #[test]
    fn test_to_little_endian() {
        let test_int = BigUint::from_str_radix("1001283637283949347689273343", 10).unwrap();
        let little_endian = test_int.to_little_endian();
        println!("little_endian: {little_endian}");
        assert_eq!(little_endian, "ff17ed8a5ccea9bd0e3e3c03".to_string());
    }
}
