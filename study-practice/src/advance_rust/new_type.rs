/// 为外部类型实现外部特征，比如标准库中的vec和fmt
use std::fmt;
struct WrapperVec<T>(Vec<T>)
where
    T: ToString + Copy;
impl<T> fmt::Display for WrapperVec<T>
where
    T: ToString + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let target: Vec<String> = self
            .0
            .clone()
            .into_iter()
            .map(|ele| ele.to_string())
            .collect();
        write!(f, "{:?}", target)
    }
}

fn example() {
    let w1 = WrapperVec(vec!["hello", "rust", "world"]);
    println!("w1: {}", w1);
    let w2 = WrapperVec(vec![1009, 1768, 4562]);
    println!("w2: {}", w2);
}

/// type name = type_name; 别名机制
/// dst 动态大小类型
fn str_example() {
    // the trait `Sized` is not implemented for `str`
    // let s1：Box<str> = Box::new("hello there!" as str);
    // 借助编译器转换为目标的类型
    let s2: Box<str> = "hello there!".into();
    println!("{s2}");
}

#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    })=> {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }
        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    };
}

use std::convert::TryFrom;
enum MyEnum {
    A = 1,
    B,
    C,
}
impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}
fn enum_convert_match() {
    back_to_enum! {
        enum MyEnum{
            A =1,
            B,
            C,
        }
    }
}

pub fn practice() {
    example();
    str_example();
}
