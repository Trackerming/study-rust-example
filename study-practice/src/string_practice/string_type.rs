use std::io::Read;

// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
fn method_1() {
    /*
    let mut s: String = "hello, ";
    s.push_str("world".to_string());
    s.push(__);

    move_ownership(s);

    assert_eq!(s, "hello, world!");

    println!("Success!")
    */
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(&s);

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: &String) {
    println!("ownership of \"{}\" is moved here!", s)
}

// 填空
fn method_2() {
    /*
    let mut s = String::from("hello, world");
    let slice1: &str = __; // 使用两种方法
    assert_eq!(slice1, "hello, world");
    let slice2 = __;
    assert_eq!(slice2, "hello");
    let slice3: __ = __;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
    */
    let mut s = String::from("hello, world");

    // 方法1: let slice1: &str = "hello, world"; // 使用两种方法
    let slice1: &str = &s[..];
    assert_eq!(slice1, "hello, world");

    let slice2 = "hello";
    assert_eq!(slice2, "hello");

    let mut slice3: String = "hello, world".to_string();
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");

    println!("Success!")
}

// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答: 2
fn method_3() {
    // 基于 `&str` 类型创建一个 String,
    // 字符串字面量的类型是 `&str`
    // 1次生成String分配堆内存
    let s: String = String::from("hello, world!");

    // 创建一个切片引用指向 String `s`
    let slice: &str = &s;

    // 2次
    // 基于刚创建的切片来创建一个 String
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

// 填空并修复错误
fn method_4() {
    /*
    let s = String::from("hello, 世界");
    let slice1 = s[0]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    assert_eq!(slice1, "h");
    let slice2 = &s[3..5];// 提示: `世` 在 UTF-8 编码中占用 3 个字节
    assert_eq!(slice2, "世");
    // 迭代 s 中的所有字符
    for (i, c) in s.__ {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }
    println!("Success!")
    */
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // 提示: `世` 在 UTF-8 编码中占用 3 个字节
    assert_eq!(slice2, "世");

    // 迭代 s 中的所有字符
    let mut i = 0;
    // s.chars().enumerate()
    for c in s.chars() {
        if i == 7 {
            assert_eq!(c, '世')
        }
        i += 1;
    }

    println!("Success!")
}

// 填空
fn method_5() {
    /*
    let mut s = String::new();
    __;
    let v = vec![104, 101, 108, 108, 111];
    // 将字节数组转换成 String
    let s1 = __;
    assert_eq!(s, s1);
    println!("Success!")
    */
    let mut s = String::new();
    s = String::from("hello");

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = String::from_utf8(v).unwrap();
    println!("s1 {s1}");
    assert_eq!(s, s1);

    println!("Success!")
}

fn method_6() {
    /*
    let mut s = String::new();
    println!("{}", s.capacity());
    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }
    println!("Success!")
    */
    // 修改下面的代码以打印如下内容:
    // 25
    // 25
    // 25
    // 循环中不会发生任何内存分配
    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}

// 填空
use std::mem;

fn method_7() {
    let story = String::from("Rust By Practice");

    // 阻止 String 的数据被自动 drop
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    assert_eq!(16, len);

    // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
    // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}

pub fn practice() {
    println!("String type: run method_1: ");
    method_1();
    println!("String type: run method_2: ");
    method_2();
    println!("String type: run method_3: ");
    method_3();
    println!("String type: run method_4: ");
    method_4();
    println!("String type: run method_5: ");
    method_5();
    println!("String type: run method_6: ");
    method_6();
    println!("String type: run method_7: ");
    method_7();
}
