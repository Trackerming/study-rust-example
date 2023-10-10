/* 使用两种方法填空 */
fn method_1() {
    __;
    need_static(v);

    println!("Success!")
}
fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;
/* 让代码工作，但不要修改函数的签名 */
fn init() -> Option<&'static mut Config> {
    Some(&mut Config {
        a: "A".to_string(),
        b: "B".to_string(),
    })
}
fn method_2() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}

fn method_3() {
    {
        // 字符串字面量能跟程序活得一样久，因此 `static_string` 的生命周期是 `'static`
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // 当 `static_string` 超出作用域时，该引用就无法再被使用，但是引用指向的数据( 字符串字面量 ) 依然保存在二进制 binary 所占用的内存中
    }

    println!("static_string reference remains alive: {}", static_string);
}

// 声明一个 static 常量，它拥有 `'static` 生命周期.
static NUM: i32 = 18;
// 返回常量 `Num` 的引用，注意，这里的生命周期从 `'static` 强转为 `'a`
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}
fn method_4() {
    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

/* 让代码工作 */
use std::fmt::Debug;
fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}
fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}
fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}
fn method_5() {
    // i 是有所有权的数据，并没有包含任何引用，因此它是 'static
    let i = 5;
    print_it(i);

    // 但是 &i 是一个引用，生命周期受限于作用域，因此它不是 'static
    print_it(&i);

    print_it1(&i);

    // 但是下面的代码可以正常运行 !
    print_it2(&i);
}

use std::fmt::Display;
fn method_6() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(&string); // Compilation error
    print_d(&string); // Compilation error
    print_e(&string);
    print_f(&string);
    print_g(&string); // Compilation error
}
fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}
fn print_b<T>(t: &T)
where
  T: Display + 'static,
{
    println!("{}", t);
}
fn print_c(t: &'static dyn Display) {
    println!("{}", t)
}
fn print_d(t: &'static impl Display) {
    println!("{}", t)
}
fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}
fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}
fn print_g(t: &'static String) {
    println!("{}", t);
}

pub fn practice(){
    println!("Advanced Rust Lifttime staticp practice");
    method_1();
}
