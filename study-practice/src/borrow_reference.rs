fn method_1() {
    /*
      let x = 5;
      // 填写空白处
      let p = __;
      println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
    */
    let x = 5;
    // 填写空白处
    let p = &x;
    println!("x 的内存地址是 {:p}", p); // output: 0x16fa3ac84
}

fn method_2() {
    /*
    let x = 5;
    let y = &x;
    // 只能修改以下行
    assert_eq!(5, y);
    */
    let x = 5;
    let y = &x;

    // 只能修改以下行
    assert_eq!(5, *y);
}

// 修复错误
fn method_3() {
    /*
    let mut s = String::from("hello, ");
    borrow_object(s)
    */
    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(s: &String) {}

/// 可变引用 修复错误
fn method_4() {
    /*
    let mut s = String::from("hello, ");
    push_str(s)
    */
    let mut s = String::from("hello, ");

    push_str(&mut s);
    println!("s: {}", s);
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

fn method_5() {
    /*
    let mut s = String::from("hello, ");
    // 填写空白处，让代码工作
    let p = __;
    p.push_str("world");
    */
    let mut s = String::from("hello, ");
    // 填写空白处，让代码工作
    let p = &mut s;
    p.push_str("world");
    println!("p: {}", p);
}

fn method_6() {
    /*
    let c = '中';
    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let __ r2 = c;
    assert_eq!(*r1, *r2);
    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1),get_addr(r2));
    */
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c;
    assert_eq!(*r1, *r2);

    // 判断两个内存地址的字符串是否相等
    println!("r1 addr: {:p}", r1);
    println!("r2 addr: {:p}", r2);
    assert_eq!(get_addr(r1), get_addr(r2));
}

// 获取传入引用的内存地址的字符串形式
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// 移除代码某个部分，让它工作
// 你不能移除整行的代码！
fn method_7() {
    /*
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
    */
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
}

fn method_8() {
    /*
    // 通过修改下面一行代码来修复错误
    let s = String::from("hello, ");
    borrow_object_mut(&mut s)
    */
    // 通过修改下面一行代码来修复错误
    let mut s = String::from("hello, ");

    borrow_object_mut(&mut s)
}

fn borrow_object_mut(s: &mut String) {}

// 下面的代码没有任何错误
fn method_9() {
    let mut s = String::from("hello, ");

    borrow_object_9(&s);

    s.push_str("world");
}

fn borrow_object_9(s: &String) {}

// 注释掉一行代码让它工作
fn method_10() {
    /*
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    println!("{}",r1);
    */
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    //println!("{}",r1);
}

fn method_11() {
    /*
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    */
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // 从这里的scope1开始不能使用r1.否则r2借用不成功

    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    // println!("r1: {}, r2: {}", r1, r2);
}

fn unimplement() -> ! {
    unimplemented!();
}

pub fn practice() {
    println!("run method_1: ");
    method_1();
    println!("run method_2: ");
    method_2();
    println!("run method_3: ");
    method_3();
    println!("run method_4: ");
    method_4();
    println!("run method_5: ");
    method_5();
    println!("run method_6: ");
    method_6();
    println!("run method_7: ");
    method_7();
    println!("run method_8: ");
    method_8();
    println!("run method_9: ");
    method_9();
    println!("run method_10: ");
    method_10();
    println!("run method_11: ");
    method_11();
    // unimplement();
}
