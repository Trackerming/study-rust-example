// 填空
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        __
    }

    println!("Exercise Failed if printing out this line!");
}
fn method_1() {
    drink(__);

    println!("Exercise Failed if printing out this line!");
}

// 修复所有的 panic，让代码工作
fn method_2() {
    assert_eq!("abc".as_bytes(), [96, 97, 98]);

    let v = vec![1, 2, 3];
    let ele = v[3];
    let ele = v.get(3).unwrap();

    // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
    let v = production_rate_per_hour(2);

    divide(15, 0);

    println!("Success!")
}
fn divide(x:u8, y:u8) {
    println!("{}", x / y)
}
fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => (speed * cph) as f64,
        5..=8 => (speed * cph) as f64 * 0.9,
        9..=10 => (speed * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}

pub fn practice() {
    println!("ErrorResult Panic practice method1:");
    method_1();
    println!("ErrorResult Panic practice method2:");
    method_2();
}