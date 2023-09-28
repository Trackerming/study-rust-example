#![allow(incomplete_features)]
/*#![feature(generic_const_exprs)]
fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}
// 修复 main 函数中的错误
fn method_10() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
    check_size([(); 31].map(|_| "hello你好".to_string()));  // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
    check_size(['中'; 191]); // A char takes 4 bytes in Rust
}*/
/// generic

// 填空
struct A; // 具体的类型 `A`.
struct S(A); // 具体的类型 `S`.
struct SGen<T>(T); // 泛型 `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn method_1() {
    // 使用非泛型函数
    reg_fn(S(A)); // 具体的类型
    gen_spec_t(SGen(A)); // 隐式地指定类型参数  `A`.
    gen_spec_i32(SGen(32)); // 隐式地指定类型参数`i32`.

    // 显式地指定类型参数 `char`
    generic::<char>(SGen('s'));

    // 隐式地指定类型参数 `char`.
    generic(SGen('t'));
}

// 实现下面的泛型函数 sum
fn sum<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn method_2() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}

// 实现一个结构体 Point 让代码工作
fn method_3() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// 修改以下结构体让代码工作
struct Point<T, U> {
    x: T,
    y: U,
}

fn method_4() {
    // 不要修改这行代码！
    let p = Point {
        x: 5,
        y: "hello".to_string(),
    };
}

// 为 Val 增加泛型参数，不要修改 `main` 中的代码
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn method_5() {
    let x = Val { val: 3.0 };
    let y = Val {
        val: "hello".to_string(),
    };
    println!("{}, {}", x.value(), y.value());
}

struct Point6<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point6<T, U> {
    // 实现 mixup，不要修改其它代码！
    fn mixup<K, V>(&self, other_point: Point6<K, V>) -> Point6<T, V>
    where
        T: std::marker::Copy,
    {
        Point6 {
            x: self.x,
            y: other_point.y,
        }
    }
}

fn method_6() {
    let p1 = Point6 { x: 5, y: 10 };
    let p2 = Point6 {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
}

// 修复错误，让代码工作
struct Point7<T> {
    x: T,
    y: T,
}

impl Point7<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn method_7() {
    let p = Point7 {
        x: 5 as f32,
        y: 10 as f32,
    };
    println!("{}", p.distance_from_origin())
}

/// const

// 修复错误
struct Array<T, const N: usize> {
    data: [T; N],
}

fn method_8() {
    let arrays = [
        Array { data: [1, 2, 3] },
        Array { data: [1, 2, 5] },
        Array { data: [1, 2, 6] },
    ];
}

// 填空
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn method_9() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

pub enum Assert<const CHECK: bool> {}
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

pub fn practice() {
    println!("Generic&Const run method_1: ");
    method_1();
    println!("Generic&Const run method_2: ");
    method_2();
    println!("Generic&Const run method_3: ");
    method_3();
    println!("Generic&Const run method_4: ");
    method_4();
    println!("Generic&Const run method_5: ");
    method_5();
    println!("Generic&Const run method_6: ");
    method_6();
    println!("Generic&Const run method_7: ");
    method_7();
    println!("Generic&Const run method_8: ");
    method_8();
    println!("Generic&Const run method_9: ");
    method_9();
}
