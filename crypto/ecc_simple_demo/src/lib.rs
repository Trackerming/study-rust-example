pub mod point;

use crate::point::{mod_exp, Point};
use std::collections::HashMap;
use std::ops::Deref;

/// SEGMAX是啥？
/// 加解密的验证过程
/// y^2 = x^3+ax+b
/// 假设曲线为 y^2 = x^3 + x + 1 （mod 23）基点 G(0, 1)
/// mod_value和阶的关系；
pub struct ECC23 {
    G: Point,
    a: usize,
    b: usize,
    // 限定了x和y都只能在0～mod_value之间的取值范围
    mod_value: usize,
    n: usize,
}

const A: usize = 1;

impl ECC23 {
    pub fn new(g: Point, a: usize, b: usize, mod_value: usize) -> Self {
        let G = g;
        let mut pointers = HashMap::new();
        pointers.insert(1, G.clone());
        Self {
            G,
            a,
            b,
            mod_value,
            n: 0,
        }
    }

    pub fn point_addition(&self, p: Point, q: Point) -> Point {
        if p.eq(&Point { x: 0, y: 0 }) {
            return q;
        }
        if q.eq(&Point { x: 0, y: 0 }) {
            return p;
        }
        // 计算两点的斜率
        let m: usize;
        if p != q {
            m = ((q.y + self.mod_value - p.y)
                * mod_exp(
                    q.x + self.mod_value - p.x,
                    self.mod_value - 2,
                    self.mod_value,
                ))
                % self.mod_value;
        } else {
            m = ((3 * p.x.pow(2) + A) * mod_exp(2 * p.y, self.mod_value - 2, self.mod_value))
                % self.mod_value;
        }
        let mut point_result = Point { x: 0, y: 0 };
        if m != 0 {
            point_result.x = (m.pow(2) + self.mod_value - p.x - q.x) % self.mod_value;
            point_result.y = (m * (p.x + self.mod_value - point_result.x) + self.mod_value - p.y)
                % self.mod_value;
        } else {
            point_result.x = (2 * self.mod_value - p.x - q.x) % self.mod_value;
            point_result.y = (self.mod_value - p.y) % self.mod_value;
        }
        point_result
    }

    pub fn scalar_multiplication(&self, d: usize) -> Point {
        let mut result = Point { x: 0, y: 0 };
        let mut current = self.G;
        let mut d = d;
        while d > 0 {
            if d % 2 == 1 {
                result = self.point_addition(result, current);
            }
            current = self.point_addition(current, current);
            d >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_point() {
        let expect_points = [
            Point { x: 0, y: 1 },
            Point { x: 6, y: 19 },
            Point { x: 3, y: 13 },
            Point { x: 13, y: 16 },
            Point { x: 18, y: 3 },
            Point { x: 7, y: 11 },
            Point { x: 11, y: 3 },
            Point { x: 5, y: 19 },
            Point { x: 19, y: 18 },
            Point { x: 12, y: 4 },
            Point { x: 1, y: 16 },
            Point { x: 17, y: 20 },
            Point { x: 9, y: 16 },
            Point { x: 4, y: 0 },
            Point { x: 9, y: 7 },
            Point { x: 17, y: 3 },
            Point { x: 1, y: 7 },
            Point { x: 12, y: 19 },
            Point { x: 19, y: 5 },
            Point { x: 5, y: 4 },
            Point { x: 11, y: 20 },
            Point { x: 7, y: 12 },
            Point { x: 18, y: 20 },
        ];
        let mod_val = 23;
        let ecc23 = ECC23::new(Point { x: 0, y: 1 }, 1, 1, mod_val);
        let mut points = vec![];
        for i in 1..mod_val + 1 {
            let point = ecc23.scalar_multiplication(i);
            println!("{i}G: {:?}", point);
            points.push(point);
        }
        assert_eq!(points, expect_points);
    }
}
