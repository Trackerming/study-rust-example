#[derive(Debug)]
enum List {
    // 定义为了Box，存了指向数据的指针，指针的大小也是确定的
    Cons(i32, Box<List>),
    Nil,
}

fn store_base_type_on_heap() {
    // 正常i32会存储在栈上，这样将它存储在heap上；
    let b = Box::new(180);
    println!("b = {}", b);
}

/*
    为了确定为 Message 值分配多少空间，Rust 会遍历每个变体以查看哪个变体需要最多空间。
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColer(i32, i32, i32),
}

pub fn box_pointer_study() {
    store_base_type_on_heap();
    // 不加Box::new()会出现error[E0072]: recursive type `List` has infinite size 编译错误
    // let list2 = List::Cons(32, List::Cons(18, List::Nil));
    let list = Box::new(List::Cons(
        32,
        Box::new(List::Cons(
            24,
            Box::new(List::Cons(36, Box::new(List::Nil))),
        )),
    ));
    println!("list: {:?}", list);
    // 释放内存的时候，指针和指向指针的数据都会被释放；
}
