/// 运行结果如下
/// ```txt
/// two
/// ```
fn match_literals() {
    let x = 2;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/// 运行结果如下
/// ```txt
/// Matched, y = 10
/// end, x = Some(10), y = 5
/// ```
fn match_named_variable() {
    let x = Some(10);
    let y = 5;
    match x {
        Some(50) => println!("Got 50"),
        // 引入了一个名为 y 的新变量，它将匹配 Some 值内的任何值。因为我们处于 match 表达式内的新作用域，所以这是一个新的 y 变量，而不是我们在开头声明的值为 10 的 y
        // 这个新的 y 绑定将匹配 Some 内的任何值，这就是我们在 x 中的值。因此，这个新的 y 绑定到 x 中 Some 的内部值
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("end, x = {:?}, y = {y}", x);
}

/// 运行结果如下
/// ```txt
/// one or two
/// ```
fn multi_matched() {
    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything."),
    }
}

/// 运行结果如下
/// ```txt
/// from zero to five
/// ```
fn match_range_vals() {
    let x = 4;
    match x {
        0..=5 => println!("from zero to five"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

/// 运行结果如下
/// ```txt
/// a = 0, b = 7
/// x = 0, y = 7
/// ```
fn destruct_struct() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    println!("a = {a}, b = {b}");
    println!("x = {x}, y = {y}");
}

/// 运行结果如下
/// ```txt
/// on the y axis at 7
/// ```
fn match_point() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("on the x axis at {x}"),
        Point { x: 0, y } => println!("on the y axis at {y}"),
        Point { x, y } => {
            println!("on neither axis :({x}, {y})")
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

/// 运行结果如下
/// ```txt
/// Change color to hue 0, saturation 160, value 255
/// ```
fn match_nested_struct_enum() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        Message::Move { x, y } => {
            println!("move x: {x}, y: {y}");
        }
        _ => (),
    }
}

/// 运行结果如下
/// ```txt
/// this code only use the y params: 4  
/// ```
fn test_ignore_params(_: i32, y: i32) {
    println!("this code only use the y params: {y}");
}

/// 运行结果如下
/// ```txt
/// Can't override an existing customized value
/// setting is Some(5)
/// _ ignore case: Some numbers: 1, 3
/// .. ignore case: Some numbers: 1, 7
/// ```
fn test_ignore_part_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't override an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (1, 2, 3, 4, 5, 6, 7);
    match numbers {
        (first, _, third, _, _, _, _) => {
            println!("_ ignore case: Some numbers: {first}, {third}");
        }
    }

    match numbers {
        (first, .., last) => {
            println!(".. ignore case: Some numbers: {first}, {last}");
        }
    }
}

/// 运行结果如下
/// ```txt
/// Matched, n = 10
/// end: x = Some(10) y = 10
/// ```
fn match_guard() {
    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("end: x = {:?} y = {y}", x)
}

/// 运行结果如下
/// ```txt
/// found y in range: 7, x = 34
/// ```
fn match_bindings() {
    let msg = Message::Move { x: 34, y: 7 };
    match msg {
        Message::Move {
            x,
            y: y_var @ 3..=9,
        } => println!("found y in range: {}, x = {}", y_var, x),
        _ => (),
    }
}

pub fn pattern_syntax_study() {
    match_literals();
    match_named_variable();
    multi_matched();
    match_range_vals();
    destruct_struct();
    match_point();
    match_nested_struct_enum();
    test_ignore_params(3, 4);
    test_ignore_part_value();
    match_guard();
    match_bindings();
}
