/// 运行结果如下
/// ```txt
/// value = 32
/// ```
fn match_arm() {
    let var: Option<i32> = Some(32);
    match var {
        Some(value) => println!("value = {value}"),
        _ => println!("value is ignored."),
    }
}

/// 重点在匹配上，运行结果如下
/// ```txt
/// Using purple as the background color
/// ```
fn if_let_usage() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background.");
    } else if is_tuesday {
        println!("tuesday is green day.");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

/// 运行结果如下
/// ```txt
/// current pop: 3
/// current pop: 2
/// current pop: 1
/// ```
fn while_let_loop_usage() {
    let mut vec_val = vec![1, 2, 3];
    while let Some(pop) = vec_val.pop() {
        println!("current pop: {}", pop);
    }
}

/// 运行结果为
/// ```txt
/// index 0 value a
/// index 1 value b
/// index 2 value c
/// ```
fn for_loop_usage() {
    let v = vec!['a', 'b', 'c'];
    // 使用 enumerate 方法调整迭代器，以便它生成一个值以及该值的索引，并将其放入元组中。生成的第一个值是元组 (0, 'a') 。
    // 当此值与模式 (index, value) 匹配时， index 将是 0 ， value 将是 'a'
    for (index, value) in v.iter().enumerate() {
        println!("index {} value {}", index, value);
    }
}

pub fn pattern_usage_places_study() {
    match_arm();
    if_let_usage();
    while_let_loop_usage();
    for_loop_usage();
}
