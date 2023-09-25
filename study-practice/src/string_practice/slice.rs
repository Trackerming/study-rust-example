// 修复代码中的错误，不要新增代码行!
fn method_1() {
    /*
    let arr = [1, 2, 3];
    let s1: [i32] = arr[0..2];
    let s2: str = "hello, world" as str;
    */
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;
}

fn method_2() {
    /*
    let arr: [char; 3] = ['中', '国', '人'];
    let slice = &arr[..2];
    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    assert!(std::mem::size_of_val(&slice) == 8);
    */
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    let size = std::mem::size_of_val(&slice);
    println!("{size}");

    // 修改数字 `8` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
    assert!(std::mem::size_of_val(&slice) == 16);
    todo!("为什么是16");
}

fn method_3() {
    /*
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 填空让代码工作起来
    let slice: __ = __;
    assert_eq!(slice, &[2, 3, 4]);
    */
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // 填空让代码工作起来
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}

fn method_4() {
    /*
    let s = String::from("hello");
    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[__];
    assert_eq!(slice1, slice2);
    */
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}

fn method_5() {
    /*
    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    let slice = &s[0..2];
    assert!(slice == "你");
    */
    let s = "你好，世界";
    // 修改以下代码行，让代码工作起来
    let slice = &s[0..3];

    assert!(slice == "你");
}

// 修复所有错误
fn method_6() {
    /*
    let mut s = String::from("hello world");
    // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let ch = first_character(&s);
    s.clear(); // error!
    println!("the first character is: {}", ch);
    */
    let mut s = String::from("hello world");

    // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let ch = first_character(&s);
    println!("the first character is: {}", ch);
    s.clear(); // error!
}
fn first_character(s: &str) -> &str {
    &s[..1]
}

pub fn practice() {
    println!("slice: run method_1: ");
    method_1();
    println!("slice: run method_3: ");
    method_3();
    println!("slice: run method_4: ");
    method_4();
    println!("slice: run method_5: ");
    method_5();
    println!("slice: run method_6: ");
    method_6();
    //println!("slice: run method_2: ");
    //method_2();
}
