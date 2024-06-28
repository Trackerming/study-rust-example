fn outside_scope() {
    let r;
    //{
    let x = 5;
    // error[E0597]: `x` does not live long enough
    // 解法将x的生命周期拓展到一样的函数体
    r = &x;
    //}
    println!("r {}", r);
}

/*
error[E0106]: missing lifetime specifier
  --> bin/generic_traits_lifetime/lifetime_study.rs:12:38
   |
12 | fn longer(str1: &str, str2: &str) -> &str {
   |                 ----        ----     ^ expected named lifetime parameter
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `str1` or `str2`
help: consider introducing a named lifetime parameter
   |
12 | fn longer<'a>(str1: &'a str, str2: &'a str) -> &'a str {
   |          ++++        ++             ++          ++
*/
// fn longer(str1: &str, str2: &str) -> &str {
fn longer<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() >= str2.len() {
        str1
    } else {
        str2
    }
}

struct Score<'a> {
    name: &'a str,
    score: &'a i32,
}

impl<'a> Score<'a> {
    // [规则3]：因为参数之一是 &self ，所以返回类型获取 &self 的生命周期
    fn get_score(&self) -> &i32 {
        self.score
    }
}

// 能编译过的原因是1.0版本以后，引入了生命周期省略原则，对一些通用确定的情况可以省略繁琐的定义
fn first_world(str: &str) -> &str {
    for (index, &item) in str.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &str[..index];
        }
    }
    return &str;
}
/*
    [rule1] 转换为：fn first_world<'a>(str: &'a str) -> &str {
    [rule2] 转换为：fn first_world<'a>(str: &'a str) -> &'a str {
*/

pub fn lifetime_kown_study<'a>() {
    outside_scope();
    let num1: &i32;
    let num2: &'a i32;
    let num3: &'a mut i32;
    let str_result: &str;
    let str1 = String::from("hello rust lifetime.");
    {
        let str2 = String::from("world.");
        str_result = longer(&str1, &str2);
        println!("str longer: {}", str_result);
    }
    // error[E0597]: `str2` does not live long enough
    // println!("str_result: {}", str_result);

    let s: &'static str = "test 'static";
}
