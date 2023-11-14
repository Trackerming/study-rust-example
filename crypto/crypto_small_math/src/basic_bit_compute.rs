pub fn xor_compute_u8(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn and_compute_u8(a: u8, b: u8) -> u8 {
    a & b
}

pub fn or_compute_u8(a: u8, b: u8) -> u8 {
    a | b
}

pub fn nor_compute_u8(a: u8) -> u8 {
    !a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xor_compute_u8() {
        let a = 8;
        let b = 4;
        let c = super::xor_compute_u8(a, b);
        assert_eq!(c, 12);
    }

    #[test]
    fn and_compute_u8() {
        let a = 8;
        let b = 4;
        let c = super::and_compute_u8(a, b);
        assert_eq!(c, 0);
    }

    #[test]
    fn or_compute_u8() {
        let a = 8;
        let b = 4;
        let c = super::or_compute_u8(a, b);
        assert_eq!(c, 12);
    }

    #[test]
    fn nor_compute_u8() {
        let a = 8;
        let c = super::nor_compute_u8(a);
        assert_eq!(c, 247);
    }
}
