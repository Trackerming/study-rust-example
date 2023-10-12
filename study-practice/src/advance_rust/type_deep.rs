#![allow(overflowing_literals)]
use std::rc::Rc;
use std::sync::Arc;

fn mem_convert() {
    // i32 占4Bytes
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 *= 10;
    }
    assert_eq!(values[1], 20);
    println!("values: {:?}", values);
}

// use std::convert::TryInto;
fn try_into_example() {
    let a: u8 = 10;
    let b: u16 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b_) => b_,
        Err(msg) => {
            println!("error:{:?}", msg.to_string());
            0
        }
    };
    if a < b_ {
        println!("Ten is less than {b_} ");
    }
}

/*
1. 首先，编译器检查它是否可以直接调用 T::foo(value)，称之为值方法调用
2. 如果上一步调用无法完成(例如方法类型错误或者特征没有针对 Self 进行实现，上文提到过特征不能进行强制转换)，那么编译器会尝试增加自动引用，例如会尝试以下调用： <&T>::foo(value) 和 <&mut T>::foo(value)，称之为引用方法调用
3. 若上面两个方法依然不工作，编译器会试着解引用 T ，然后再进行尝试。这里使用了 Deref 特征 —— 若 T: Deref<Target = U> (T 可以被解引用为 U)，那么编译器会使用 U 类型进行尝试，称之为解引用方法调用
4. 若 T 不能被解引用，且 T 是一个定长类型(在编译期类型长度是已知的)，那么编译器也会尝试将 T 从定长类型转为不定长类型，例如将 [i32; 2] 转为 [i32]
*/
fn example_find_func_call() {
    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([32, 45, 87]));
    /*
    1. 首先， array[0] 只是Index特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，当然在调用之前，编译器会先检查 array 是否实现了 Index 特征。
    2. 接着，编译器检查 Rc<Box<[T; 3]>> 是否有实现 Index 特征，结果是否，不仅如此，&Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 也没有实现。
    3. 上面的都不能工作，编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
        此时继续对 Box<[T; 3]> 进行上面的操作 ：Box<[T; 3]>， &Box<[T; 3]>，和 &mut Box<[T; 3]> 都没有实现 Index 特征，
    4. 所以编译器开始对 Box<[T; 3]> 进行解引用，然后我们得到了 [T; 3]
        [T; 3] 以及它的各种引用都没有实现 Index 索引(是不是很反直觉:D，在直觉中，数组都可以通过索引访问，实际上只有数组切片才可以!)，它也不能再进行解引用，
    5. 因此编译器只能祭出最后的大杀器：将定长转为不定长，因此 [T; 3] 被转换成 [T]，也就是数组切片，它实现了 Index 特征，因此最终我们可以通过 index 方法访问到对应的元素
    */
    println!("array[0] = {}", array[0]);
}

#[derive(Clone)]
struct Container<T>(Arc<T>);
fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_container = foo.clone();
    let bar_container = bar.clone();
}

fn foo() -> i32 {
    0
}
fn unsafe_func_pointer_example() {
    let pointer = foo as *const ();
    let function = unsafe {
        // 裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    let result = function();
    println!("{result}");
}

/// as篇
// 修复错误，填空
// 不要移除任何代码
fn method_1() {
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;
    let c1: char = decimal as u8 as char;
    let c2 = integer as char;
    assert_eq!(integer, 'b' as u8 - 1);
    println!("Success!")
}

fn method_2() {
    assert_eq!(u8::MAX, 255);
    // 如上所示，u8 类型允许的最大值是 255.
    // 因此以下代码会报溢出的错误： literal out of range for `u8`.
    // **请仔细查看相应的编译错误，从中寻找到解决的办法**
    // **不要修改 main 中的任何代码**
    let v = 1000 as u8;

    println!("Success!")
}

fn method_3() {
    assert_eq!(1000 as u16, 1000);

    assert_eq!(1000 as u8, 232);

    // 事实上，之前说的规则对于正整数而言，就是如下的取模
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255);

    // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);

    // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
    // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
    // 总之，请小心使用
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

// 填空
fn method_4() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
    unsafe {
        // 将第二个元素加 1
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
    println!("Success!")
}

// need caculate
fn method_5() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe { assert_eq!(std::mem::size_of_val(&*b), 13) }
}

/// try_info
fn method_6() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // 使用两种方式修复错误
    // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
    // 2. 上一章节中介绍过的某个关键字
    let i3: u32 = 'a'.into();
    let i4: i32 = 'a' as i32;

    // 使用两种方法来解决错误
    let s: String = 'a'.into();
    // String::from('a');

    println!("Success!")
}

// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;
#[derive(Debug)]
struct Number {
    value: i32,
}
impl From<i32> for Number {
    // 实现 `from` 方法
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
// 填空
fn method_7() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);
    let num: Number = (30 as i32).into();
    assert_eq!(num.value, 30);
    println!("Success!")
}

use std::fs;
use std::io;
use std::num;
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}
impl From<io::Error> for CliError {
    // 实现 from 方法
    fn from(err: io::Error) -> Self {
        CliError::IoError(err)
    }
}
impl From<num::ParseIntError> for CliError {
    // 实现 from 方法
    fn from(err: num::ParseIntError) -> Self {
        CliError::ParseError(err)
    }
}
fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? 自动将 io::Error 转换成 CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
fn method_8() {
    println!("Success!")
}

// TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
// use std::convert::TryInto;
fn method_9() {
    let n: i16 = 256;

    // Into 特征拥有一个方法`into`,
    // 因此 TryInto 有一个方法是 ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!(
                "there is an error when converting: {:?}, but we catch it",
                e.to_string()
            );
            0
        }
    };
    assert_eq!(n, 0);

    println!("Success!")
}

#[derive(Debug, PartialEq)]
struct EvenNum(i32);
impl TryFrom<i32> for EvenNum {
    type Error = ();
    // 实现 `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}
fn method_10() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // 填空
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!")
}

use std::fmt;
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    // 实现 fmt 方法
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}
fn method_11() {
    let origin = Point { x: 0, y: 0 };
    // 填空
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success!")
}

// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
use std::str::FromStr;
fn method_12() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!")
}

use std::num::ParseIntError;
#[derive(Debug, PartialEq)]
struct Point13 {
    x: i32,
    y: i32,
}
impl FromStr for Point13 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point13 {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
fn method_13() {
    // 使用两种方式填空
    // 不要修改其它地方的代码
    let p = "(3,4)".parse::<Point13>();
    // let p1 = Point::from_str("(3, 4)");
    assert_eq!(p.unwrap(), Point13 { x: 3, y: 4 });

    println!("Success!")
}

pub fn practice() {
    mem_convert();
    try_into_example();
    example_find_func_call();
    unsafe_func_pointer_example();
    println!("Advanced Rust deep types staticp practice method1:");
    method_1();
    println!("Advanced Rust deep types staticp practice method2:");
    method_2();
    println!("Advanced Rust deep types staticp practice method3:");
    method_3();
    println!("Advanced Rust deep types staticp practice method4:");
    method_4();
    println!("Advanced Rust deep types staticp practice method5:");
    method_5();
    println!("Advanced Rust deep types staticp practice method6:");
    method_6();
    println!("Advanced Rust deep types staticp practice method7:");
    method_7();
    println!("Advanced Rust deep types staticp practice method8:");
    method_8();
    println!("Advanced Rust deep types staticp practice method9:");
    method_9();
    println!("Advanced Rust deep types staticp practice method10:");
    method_10();
    println!("Advanced Rust deep types staticp practice method11:");
    method_11();
    println!("Advanced Rust deep types staticp practice method12:");
    method_12();
    println!("Advanced Rust deep types staticp practice method13:");
    method_13();
}
