use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
