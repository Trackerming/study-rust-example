use std::hash::Hash;

pub fn mod_exp(base: usize, exponent: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
