// 修复错误
enum Number {
    Zero,
    One,
    Two,
}
enum Number1 {
    Zero = 0,
    One,
    Two,
}
// C语言风格的枚举定义
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}

fn method_1() {
    /*
    assert_eq!(Number::One, Number1::One);
    assert_eq!(Number1::One, Number2::One);
    */
    // 通过 `as` 可以将枚举值强转为整数类型
    assert_eq!(Number::One as i32, Number1::One as i32);
    assert_eq!(Number1::One as i64, Number2::One as i64);
}

// 填空
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn method_2() {
    /*
    let msg1 = Message::Move{__}; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write(__); // 使用 "hello, world!" 来初始化
    */
    let msg1 = Message::Move { x: 1, y: 2 }; // 使用x = 1, y = 2 来初始化
    let msg2 = Message::Write("hello, world!".to_string()); // 使用 "hello, world!" 来初始化
}

// 仅填空并修复错误
enum Message3 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn method_3() {
    let msg = Message3::Move { x: 1, y: 2 };
    // if let Message::Move{__} = msg {
    if let Message3::Move { x: a, y: b } = msg {
        println!("a: {a}, b: {b}");
        assert_ne!(a, b);
    } else {
        panic!("不要让这行代码运行！");
    }
}

// 填空，并修复错误
#[derive(Debug)]
enum Message4 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn method_4() {
    //  let msgs: [__] = [
    let msgs: [Message4; 3] = [
        Message4::Quit,
        Message4::Move { x: 1, y: 3 },
        Message4::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message4) {
    println!("{:?}", msg);
}

// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
fn method_5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // if let __ = six {
    if let Some(n) = six {
        println!("{}", n)
    } else {
        panic!("不要让这行代码运行！");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        /*
        __ => None,
        __ => Some(i + 1),
        */
        None => None,
        Some(i) => Some(i + 1),
    }
}

// 填空，让代码运行
enum List {
    // Cons: 链表中包含有值的节点，节点是元组类型，第一个元素是节点的值，第二个元素是指向下一个节点的指针
    Cons(u32, Box<List>),
    // Nil: 链表中的最后一个节点，用于说明链表的结束
    Nil,
}

// 为枚举实现一些方法
impl List {
    // 创建空的链表
    fn new() -> List {
        // 因为没有节点，所以直接返回 Nil 节点
        // 枚举成员 Nil 的类型是 List
        List::Nil
    }

    // 在老的链表前面新增一个节点，并返回新的链表
    // fn prepend(self, elem: u32) -> __ {
    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    // 返回链表的长度
    fn len(&self) -> u32 {
        match *self {
            // 这里我们不能拿走 tail 的所有权，因此需要获取它的引用
            /*
            List::Cons(_, __ tail) => 1 + tail.len(),
            */
            List::Cons(_, ref tail) => 1 + tail.len(),
            // 空链表的长度为 0
            List::Nil => 0,
        }
    }

    // 返回链表的字符串表现形式，用于打印输出
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                // 递归生成字符串
                // format!("{}, {}", head, tail.__())
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

fn method_6() {
    // 创建一个新的链表(也是空的)
    let mut list = List::new();

    // 添加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 打印列表的当前状态
    println!("链表的长度是: {}", list.len());
    println!("{}", list.stringify());
}

pub fn practice() {
    println!("Enum run method_1: ");
    method_1();
    println!("Enum run method_2: ");
    method_2();
    println!("Enum run method_3: ");
    method_3();
    println!("Enum run method_4: ");
    method_4();
    println!("Enum run method_5: ");
    method_5();
    println!("Enum run method_6: ");
    method_6();
}
