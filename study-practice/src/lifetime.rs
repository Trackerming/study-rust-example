#![allow(unused)]
fn example() {
    {
        let x = 5; // ----------+-- 'b
                   //           |
        let r = &x; // --+-- 'a  |
                    //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    } // ----------+
}

/* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */
// `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
// 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
fn method_1() {
    let i = 3;            // ----------------------+-- 'a
    {
        let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
                          //                                                │
        println!("borrow1: {}", borrow1); //              │
    } // `borrow1` 生命周期结束. ──────────────────────────────────┘
    {
        let borrow2 = &i; // ------------------+-- 'b

        println!("borrow2: {}", borrow2); //   |
    } // -+
} // -+

/* 像上面的示例一样，为 `r` 和 `x` 标准生命周期，然后从生命周期的角度. */
fn method_2() {
    {
        let r; // ---------+-- 'a
               //          |
        {
            //          |
            let x = 5; // -+-- 'b  |
            r = &x; //  |       |
            println!("r: {}", r); //          |
        } // -+       |
          //          |
    } // ---------+
}
/* 添加合适的生命周期标注，让下面的代码工作 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn method_3() {}

/* 使用三种方法修复下面的错误  */
// 方法1: 返回String
/*fn invalid_output<'a>() -> String {
    String::from("foo")
}*/
// 方法2 采用参数
/*fn invalid_output<'a>(str_val: &'a str) -> &'a str {
    str_val
}*/
// 方法3:采用'static
fn invalid_output() -> &'static str {
    "foo"
}
fn method_4() {}

// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* 让下面的代码工作 */
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` 活得不够久does not live long enough
    let y: &'a i32 = &12;

    // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
    // 你不能将一个小的生命周期强转成大的
}
fn method_5() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    // 这里，four 和 nice 的生命周期必须要比函数 print_refs 长

    failed_borrow();
    // `failed_borrow`  没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`
}

/* 增加合适的生命周期标准，让代码工作 */
// `i32` 的引用必须比 `Borrowed` 活得更久
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);
// 类似的，下面两个引用也必须比结构体 `NamedBorrowed` 活得更久
#[derive(Debug)]
struct NamedBorrowed<'b> {
    x: &'b i32,
    y: &'b i32,
}
#[derive(Debug)]
enum Either<'c> {
    Num(i32),
    Ref(&'c i32),
}
fn method_6() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

/* 让代码工作 */
#[derive(Debug)]
struct NoCopyType {}
#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}
fn method_7() {
    let var_a = 35;
    let example: Example;

    {
        let var_b = NoCopyType {};

        /* 修复错误 */
        example = Example {
            a: &var_a,
            b: &var_b,
        };
        println!("(Success!) {:?}", example);
    }
}

/*#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}*/

/* 修复函数的签名 */
// fn fix_me<'a:'b, 'b>(foo: &'a Example) -> &'b NoCopyType {
fn fix_me<'b>(foo: &Example<'_, 'b>) -> &'b NoCopyType {
    foo.b
}
fn method_8() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!")
}

/* 添加合适的生命周期让下面代码工作 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&'a self) -> i32 {
        3
    }
}
fn method_9() {}

/* 移除所有可以消除的生命周期标注 */
fn nput(x: &i32) {
    println!("`annotated_input`: {}", x);
}
fn pass(x: &i32) -> &i32 {
    x
}
fn longest10<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
struct Owner(i32);
impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}
struct Person<'a> {
    age: u8,
    name: &'a str,
}
enum Either10<'a> {
    Num(i32),
    Ref(&'a i32),
}
fn method_10() {}

pub fn practice() {
    println!("Lifetime practice method1:");
    method_1();
    println!("Lifetime practice method2:");
    method_2();
    println!("Lifetime practice method3:");
    method_3();
    println!("Lifetime practice method4:");
    method_4();
    println!("Lifetime practice method5:");
    method_5();
    println!("Lifetime practice method6:");
    method_6();
    println!("Lifetime practice method7:");
    method_7();
    println!("Lifetime practice method8:");
    method_8();
    println!("Lifetime practice method9:");
    method_9();
    println!("Lifetime practice method10:");
    method_10();
}
