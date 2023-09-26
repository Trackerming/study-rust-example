fn method_1() {
    /*
    let _t0: (u8,i16) = (0, -1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 填空让代码工作
    let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    */
    let _t0: (u8, i16) = (0, -1);
    // 元组的成员还可以是一个元组
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // 填空让代码工作
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}

// 修改合适的地方，让代码工作
fn method_2() {
    /*
    let t = ("i", "am", "sunface");
    assert_eq!(t.1, "sunface");
    */
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
}

// 修复代码错误
fn method_3() {
    /*
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
    */
    let too_long_tuple = ((1, 2, 3, 4, 5, 6, 7, 8), (9, 10, 11, 12, 13));
    println!("too long tuple: {:?}", too_long_tuple);
}

fn method_4() {
    /*
    let tup = (1, 6.4, "hello");
    // 填空
    let __ = tup;
    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
    */
    let tup = (1, 6.4, "hello");

    // 填空
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);
}

fn method_5() {
    /*
    let (x, y, z);
    // 填空
    __ = (1, 2, 3);
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
    */
    let (x, y, z);

    // 填空
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);
}

fn method_6() {
    /*
    // 填空，需要稍微计算下
    let (x, y) = sum_multiply(__);
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    */
    // 填空，需要稍微计算下
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

pub fn practice() {
    println!("tuple: run method_1: ");
    method_1();
    println!("tuple: run method_2: ");
    method_2();
    println!("tuple: run method_3: ");
    method_3();
    println!("tuple: run method_4: ");
    method_4();
    println!("tuple: run method_5: ");
    method_5();
    println!("tuple: run method_6: ");
    method_6();
}
