fn method_1() {
    /*
    // 修复错误，不要新增代码行
    let s: str = "hello, world";
    */
    // 修复错误，不要新增代码行
    let s: &str = "hello, world";
}

// 使用至少两种方法来修复错误
fn method_2() {
    /*
    let s: Box<str> = "hello, world".into();
    greetings(s)
    */
    let s: Box<str> = "hello, world".into();
    // 方法1: greetings(&s)
    greetings(&*s)
}

fn greetings(s: &str) {
    println!("{}", s)
}

// 填空
fn method_3() {
    /*
    let mut s = __;
    s.push_str("hello, world");
    s.push('!');
    assert_eq!(s, "hello, world!");
    */
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}

// 修复所有错误，并且不要新增代码行
fn method_4() {
    /*
    let  s = String::from("hello");
    s.push(',');
    s.push(" world");
    s += "!".to_string();
    println!("{}", s)
    */
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    //调用的std::string上的add方法，参数是&str
    s += "!";

    println!("{}", s)
}

// 填空
fn method_5() {
    /*
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.__("dogs", "cats");
    assert_eq!(s1, "I like cats")
    */
    let s = String::from("I like dogs");
    // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats")
}

// 修复所有错误，不要删除任何一行代码
fn method_6() {
    /*
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + s2;
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
    */
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.to_owned() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

// 使用至少两种方法来修复错误
fn method_7() {
    /*
    let s = "hello, world";
    greetings_string(s)
    */
    let s = "hello, world";
    // 方法1: greetings_string(s.to_string())
    greetings_string(String::from("") + s)
}

fn greetings_string(s: String) {
    println!("{}", s)
}

// 使用两种方法来解决错误，不要新增代码行
fn method_8() {
    /*
    let s = "hello, world".to_string();
    let s1: &str = s;
    */
    let s = "hello, world".to_string();
    // 方法1: let s1: &str = &s;
    let s1: &str = s.as_str();
}

fn method_9() {
    // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
    // 填空以输出 "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73__!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // 也可以使用 Unicode 形式的转义字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 还能使用 \ 来连接多行字符串
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
    println!("\"I\'m writing Rust\"");
}

fn method_11() {
    /*
    let s1 = String::from("hi,中国");
    let h = s1[0]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");

    let h1 = &s1[3..5];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");
    */
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
    assert_eq!(h1, "中");
}

fn method_12() {
    /*
    for c in "你好，世界".__ {
        println!("{}", c)
    }
    */
    // 填空，打印出 "你好，世界" 中的每一个字符
    for c in "你好，世界".chars() {
        println!("char: {}", c)
    }
    for c in String::from("你好，世界").as_bytes() {
        println!("byte: {}", c)
    }
}

pub fn practice() {
    println!("string run method_1: ");
    method_1();
    println!("string run method_2: ");
    method_2();
    println!("string run method_3: ");
    method_3();
    println!("string run method_4: ");
    method_4();
    println!("string run method_5: ");
    method_5();
    println!("string run method_6: ");
    method_6();
    println!("string run method_7: ");
    method_7();
    println!("string run method_8: ");
    method_8();
    println!("string run method_9: ");
    method_9();
    println!("string run method_11: ");
    method_11();
    println!("string run method_12: ");
    method_12();
}
