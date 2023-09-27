fn method_1() {
    match_number(32);
}
fn match_number(n: i32) {
    match n {
        // 匹配一个单独的值
        1 => println!("One!"),
        // 使用 `|` 填空，不要使用 `..` 或 `..=`
        __ => println!("match 2 -> 5"),
        // 匹配一个闭区间的数值序列
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn method_2() {
    // 填空，让 p 匹配第二个分支
    let p = Point { x: __, y: __ };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 第二个分支
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// 修复错误
enum Message {
    Hello { id: i32 },
}

fn method_3() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  3..=7,
        } => println!("id 值的范围在 [3, 7] 之间: {}", id),
        Message::Hello { id: newid@10 | 11 | 12 } => {
            println!("id 值的范围在 [10, 12] 之间: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// 填空让代码工作，必须使用 `split`
fn method_4() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) __ => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}

// 填空，让代码工作
fn method_5() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        __ => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
}

// 修复错误，尽量少地修改代码
// 不要移除任何代码行
fn method_6() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        &mut value => value.push_str(" world!") 
    }
}

pub fn practice() {
    println!("Match all run method_1: ");
    method_1();
    println!("Match all run method_2: ");
    method_2();
    println!("Match all run method_3: ");
    method_3();
    println!("Match all run method_4: ");
    method_4();
    println!("Match all run method_5: ");
    method_5();
    println!("Match all run method_6: ");
    method_6();
}
