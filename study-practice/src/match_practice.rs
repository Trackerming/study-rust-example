/// 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match

// 填空
enum Direction {
    East,
    West,
    North,
    South,
}
fn method_1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        __  => { // 在这里匹配 South 或 North
            println!("South or North");
        },
        _ => println!(__),
    };
}

fn method_2() {
    let boolean = true;

    // 使用 match 表达式填空，并满足以下条件
    //
    // boolean = true => binary = 1
    // boolean = false => binary = 0
    let binary = __;

    assert_eq!(binary, 1);
}


// 填空
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn method_3() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    match msg {
        __ => { // 这里匹配 Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, __);
            assert_eq!(b, __);
        }
        __ => println!("no data in these variants")
    }
}


fn method_4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // 使用 `matches` 填空
    for ab in alphabets {
        assert!(__)
    }
} 

enum MyEnum {
    Foo,
    Bar
}

fn method_5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { // 修复错误，只能修改本行代码
            count += 1;
        }
    }

    assert_eq!(count, 2);
}

fn method_6() {
    let o = Some(7);

    // 移除整个 `match` 语句块，使用 `if let` 替代
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    };
}

// 填空
enum Foo {
    Bar(u8)
}

fn method_7() {
    let a = Foo::Bar(1);

    __ {
        println!("foobar 持有的值是: {}", i);
    }
}

enum Foo8 {
    Bar,
    Baz,
    Qux(u32)
}

fn method_8() {
    let a = Foo8::Qux(10);

    // 移除以下代码，使用 `match` 代替
    if let Foo8::Bar = a {
        println!("match foo::bar")
    } else if let Foo8::Baz = a {
        println!("match foo::baz")
    } else {
        println!("match others")
    }
}

// 就地修复错误
fn method_9() {
    let age = Some(30);
    if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
        assert_eq!(age, Some(30));
    } // 新的 `age` 变量在这里超出作用域

    match age {
        // `match` 也能实现变量遮蔽
        Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
        _ => ()
    }
}

pub fn practice() {
    println!("Match run method_1: ");
    method_1();
    println!("Match run method_2: ");
    method_2();
    println!("Match run method_3: ");
    method_3();
    println!("Match run method_4: ");
    method_4();
    println!("Match run method_5: ");
    method_5();
    println!("Match run method_6: ");
    method_6();
    println!("Match run method_7: ");
    method_7();
    println!("Match run method_8: ");
    method_8();
    println!("Match run method_9: ");
    method_9();
}
