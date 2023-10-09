use std::error::Error;
// 填空并修复错误
use std::num::ParseIntError;
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}
fn method_1() {
    let result = multiply("10", "2").unwrap();
    assert_eq!(result, 20);

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!")
}

// 使用 `?` 来实现 multiply
// 不要使用 unwrap !
fn multiply_for_2(n1_str: &str, n2_str: &str) -> std::result::Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)
}
fn method_2() {
    assert_eq!(multiply_for_2("3", "4").unwrap(), 12);
    println!("Success!")
}

use std::fs::File;
use std::io::{self, Read, Result};
fn read_file1() -> std::result::Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s),
        Err(e) => return Err(e),
    }
}
// 填空
// 不要修改其它代码
fn read_file2() -> std::result::Result<String, io::Error> {
    let mut s = String::new();

    s = read_file1()?;

    Ok(s)
}
fn method_3() {
    assert_eq!(
        read_file1().unwrap_err().to_string(),
        read_file2().unwrap_err().to_string()
    );
    println!("Success!")
}

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> std::result::Result<i32, ParseIntError> {
    n_str.parse::<i32>().and_then(|x| Ok(x + 2))
}
fn method_4() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}

// 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
// 但是这种写法实在过于啰嗦..
fn multiply_for_5(n1_str: &str, n2_str: &str) -> std::result::Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1) => match n2_str.parse::<i32>() {
            Ok(n2) => Ok(n1 * n2),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
// 重写上面的 `multiply` ，让它尽量简介
// 提示：使用 `and_then` 和 `map`
fn multiply1_for_5(n1_str: &str, n2_str: &str) -> std::result::Result<i32, ParseIntError> {
    // 实现...
    n1_str
        .parse::<i32>()
        .and_then(|n1| Ok(n1 * n2_str.parse::<i32>()?))
}
fn print5(result: std::result::Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
fn method_5() {
    let twenty = multiply1_for_5("10", "2");
    print5(twenty);

    // 下面的调用会提供更有帮助的错误信息
    let tt = multiply_for_5("t", "2");
    print5(tt);

    println!("Success!")
}

// 填空
type Res<T> = std::result::Result<T, ParseIntError>;
// 使用上面的别名来引用原来的 `Result` 类型
fn multiply_for_6(first_number_str: &str, second_number_str: &str) -> Res<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

// 同样, 这里也使用了类型别名来简化代码
fn print6(result: Res<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn method_6() {
    print6(multiply_for_6("10", "2"));
    print6(multiply_for_6("t", "2"));

    println!("Success!")
}

pub fn practice() {
    println!("ErrorResult Result practice method1:");
    method_1();
    println!("ErrorResult Result practice method2:");
    method_2();
    println!("ErrorResult Result practice method3:");
    method_3();
    println!("ErrorResult Result practice method4:");
    method_4();
    println!("ErrorResult Result practice method5:");
    method_5();
    println!("ErrorResult Result practice method6:");
    method_6();
}
