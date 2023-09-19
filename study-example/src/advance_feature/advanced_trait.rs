/*pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {}
}

pub trait Iterator_Generic<T> {
    fn next(&mut self) -> Option<T>;
}*/

use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Newtype
#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

/*
ADD定义如下
trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs:Rhs) -> Self::Output;
}
*/

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

/// 运行结果为
/// ```txt
/// point: (Point { x: 0, y: 0 })
/// millis: (Millimeters(4123))
/// ```
fn operator_overload() {
    let point = Point { x: 1, y: -1 } + Point { x: -1, y: 1 };
    println!("point: ({:?})", point);
    let millis = Millimeters(123) + Meters(4);
    println!("millis: ({:?})", millis);
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot: this is your captain speking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard: Up");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

/// 运行结果为
/// ```txt
/// *waving arms furiously*
/// Pilot: this is your captain speking.
/// Wizard: Up
/// ```
fn same_fn_name_call() {
    let person = Human;
    person.fly();
    // 对于没有第一个self参数的同名函数，需要显式指定实现的结构的类型如`<Dog as Animal>::baby_name()`，否则编译器会报错
    Pilot::fly(&person);
    Wizard::fly(&person);
}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// error[E0277]: `advanced_trait::Point` doesn't implement `std::fmt::Display`
impl OutlinePrint for Point {}
// fix如下
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// 运行结果如下
/// ```txt
/// ***********
/// *         *
/// * -23, 56 *
/// *         *
/// ***********
/// ```
fn super_trait_example() {
    let point = Point { x: -23, y: 56 };
    point.outline_print();
}

struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/// 运行结果如下
/// ```txt
/// w: [Hello, Rust]
/// ```
fn newtype_pattern_external_trait() {
    let w = Wrapper(vec![String::from("Hello"), String::from("Rust")]);
    println!("w: {}", w);
}

pub fn advanced_trait_study() {
    operator_overload();
    same_fn_name_call();
    super_trait_example();
    newtype_pattern_external_trait();
}
