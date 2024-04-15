use crate::base_compute::mod_inverse;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    // 点的加法定义
    pub fn add(&self, other: &Point, mod_value: usize) -> Point {
        // 计算斜率lambda = (y2-y1)/(x2-x1)
        let mut lambda = ((other.y - self.y + mod_value) % mod_value)
            * mod_inverse((other.x - self.x + mod_value) % mod_value, mod_value)
            % mod_value;
        if lambda < 0 {
            lambda += mod_value;
        }
        // x = λ^2-x1-x2
        let x3 = (lambda * lambda - self.x - other.x) % mod_value;
        // 通过斜率 lambda 乘以第一个点的横坐标减去新点的横坐标，再减去第一个点的纵坐标得到的
        let y3 = (lambda * (self.x - x3) - self.y) % mod_value;
        Point { x: x3, y: y3 }
    }

    pub fn mul(&self, scalar: usize, mod_value: usize) -> Point {
        let mut result = Point {
            x: self.x,
            y: self.y,
        };
        for _ in 1..scalar {
            result = result.add(&self, mod_value);
        }
        result
    }
}
