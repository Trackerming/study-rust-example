use std::collections::HashSet;

// 群的几个关键特性
// 1. 非空，
// 2. 二元运算，比如加法或者乘法运算
// 3. 封闭性，集合中的元素通过运算之后仍然为群中的元素
// 4. 结合性，如(a+b)+c = a+(b+c)
// 5. 单位元e，a+e = e+a = a
// 6. 每个元素都有逆元；即a+a^-1 = e;
pub struct AddPrimeGroup {
    // 素数群中的元素个数，也是这个群的阶，大于0
    n: usize,
    // 计算取模的值
    mod_num: i8,
    // 单位元
    e: i8,
    // 生成元，先只用一个表示，群内可能不止一个生成元
    g: i8,
    // 群内的元素集合
    elems: HashSet<i8>,
}

impl AddPrimeGroup {
    pub fn new(mod_num: i8, e: i8, g: i8) -> AddPrimeGroup {
        AddPrimeGroup {
            n: 0,
            mod_num,
            e,
            g,
            elems: HashSet::new(),
        }
    }

    fn add_mod(&self, value: i8) -> i8 {
        (self.g + value) % self.mod_num
    }

    pub fn generate_elems(&mut self) {
        let mut elem = self.e;
        loop {
            elem = self.add_mod(elem);
            match self.elems.get(&elem) {
                // 已经存在生成结束，再生成也是原有的数据的循环
                Some(_) => {
                    self.n = self.elems.len();
                    break;
                }
                None => {
                    let elem_add_e = elem + self.e;
                    assert_eq!(elem_add_e, elem);
                    self.elems.insert(elem);
                }
            }
        }
    }
}

// 群的几个关键特性
// 1. 非空，
// 2. 二元运算，比如加法或者乘法运算
// 3. 封闭性，集合中的元素通过运算之后仍然为群中的元素
// 4. 结合性，如(a*b)*c = a*(b*c)
// 5. 单位元e，a*e = e*a = a
// 6. 每个元素都有逆元；即a*a^-1 = e;
pub struct MulPrimeGroup {
    // 素数群中的元素个数，也是这个群的阶，大于0
    n: usize,
    // 计算取模的值
    mod_num: i8,
    // 单位元
    e: i8,
    // 生成元，先只用一个表示，群内可能不止一个生成元
    g: i8,
    // 群内的元素集合
    elems: HashSet<i8>,
}

impl MulPrimeGroup {
    fn new(mod_num: i8, e: i8, g: i8) -> Self {
        MulPrimeGroup {
            mod_num,
            e,
            g,
            n: 0,
            elems: HashSet::new(),
        }
    }

    /// 安全计算应该
    /// ```math
    /// (self.g % self.mod_num * value % self.mod_num) % self.mod_num
    /// ```
    fn multiply_mod(&self, value: i8) -> i8 {
        (self.g * value) % self.mod_num
    }

    pub fn generate_elems(&mut self) {
        let mut elem = self.e;
        loop {
            elem = self.multiply_mod(elem);
            match self.elems.get(&elem) {
                Some(_) => {
                    self.n = self.elems.len();
                    break;
                }
                None => {
                    let elem_add_e = elem * self.e;
                    assert_eq!(elem_add_e, elem);
                    self.elems.insert(elem);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_prime_group() {
        let mut add_prime_group_of_7 = AddPrimeGroup::new(7, 0, 2);
        add_prime_group_of_7.generate_elems();
        assert_eq!(
            add_prime_group_of_7.elems,
            HashSet::from([0, 1, 2, 3, 4, 5, 6])
        )
    }

    #[test]
    fn test_mul_prime_group() {
        let mut mul_prime_group_of_7 = MulPrimeGroup::new(7, 1, 3);
        mul_prime_group_of_7.generate_elems();
        assert_eq!(
            mul_prime_group_of_7.elems,
            HashSet::from([1, 2, 3, 4, 5, 6])
        )
    }
}
