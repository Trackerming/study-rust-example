use std::fmt::Write;
pub fn u8_array_convert_string(arr: &[u8]) -> String {
    let mut result = String::new();
    for a in arr {
        write!(result, "{:02x}", a);
    }
    result
}

pub fn hex_string_2_array(param: &str) -> Vec<u8> {
    assert_eq!(param.len() % 2, 0);
    let mut result = Vec::new();
    for i in 0..(param.len() / 2) {
        let s = &param[i * 2..(i + 1) * 2];
        result.push(u8::from_str_radix(s, 16).unwrap());
    }
    result
}
