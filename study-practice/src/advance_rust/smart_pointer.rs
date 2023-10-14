/// Box的几种使用情况
/// 1. 将数据指定存储在堆上
/// 2. 较大类型的数据不想在转移所有权的时候拷贝，借用堆的指针复制进行拷贝
/// 3. 编译期大小不定的类型
/// 4. 特征对象
fn example_box_in_heap() {
    // 正常let a = 3;a应该存储在栈上
    let a = Box::new(3);
    println!("heap int data: {a}");
    let _b = *a + 1;
}
fn example_copy_in_box() {
    let array = [0; 20];
    // arary分配在栈上，所以这里拷贝了一份数据，所有权也没有转移，下面两个数组都可以使用
    let array1 = array;
    let array2: Vec<i32> = array1.iter().map(|x| x + 1).collect();
    println!("{:?}", array);
    println!("{:?}", array1);
    println!("{:?}", array2);
    let arr = Box::new([0; 20]);
    // 所有权转移给了新的arr1，所以更高效一些
    let arr1 = arr;
    // borrow moved value
    // println!("arr: {}", arr.len());
    println!("box copy arr1: {}", arr1.len());
}
//嵌套递归类型，DST不知道size，所以将list存储在堆上，采用指针指向它，指针的大小固定；
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
// 特征对象
trait Draw {
    fn draw(&self);
}
struct Select {
    id: u32,
}
impl Draw for Select {
    fn draw(&self) {
        println!("Select draw {}", self.id);
    }
}
struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Button draw {}", self.id);
    }
}
fn example_trait_object_box() {
    let eles: Vec<Box<dyn Draw>> = vec![Box::new(Select { id: 32 }), Box::new(Button { id: 32 })];
    for e in eles {
        e.draw();
    }
}
fn example_box() {
    example_box_in_heap();
    example_copy_in_box();
    let list = Box::new(List::Cons(
        12,
        Box::new(List::Cons(24, Box::new(List::Nil))),
    ));
    println!("List: {:?}", list);
    example_trait_object_box();
}
fn get_static_str() -> &'static str {
    let mut s = Box::new(String::new());
    (*s).push_str("hello rust!");
    Box::leak(s)
}
/// box leak将运行时的参数转换为'static的变量
fn example_leak_of_box() {
    let static_str = get_static_str();
    println!("static str: {static_str}");
}

/// 实现指针的Deref，以创建MyBox为例
#[derive(Debug)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
use std::ops::DerefMut;
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// Box<String> -> String -> &str 隐式转换链条
fn example_mybox() {
    let my_box_i32 = MyBox::new(32);
    let mut my_box_string = MyBox::new("hello world".to_string());
    // *操作实际上等同于背后执行 *(y.deref())
    let num = *my_box_i32 + 1;
    println!("my_box_i32: {:?}", my_box_i32);
    println!("num: {num}; my_box_string: {}.", *my_box_string);
    (*my_box_string).push_str(" rust");
    println!("num: {num}; my_box_string: {}.", *my_box_string);
}

/// 当 T: Deref<Target=U>，可以将 &T 转换成 &U，也就是我们之前看到的例子
/// 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
/// 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
use std::rc::Rc;
fn foo(str_val: &str) {}
struct Foo;
impl Foo {
    fn foo(&self, i: u32) {
        println!("Foo struct: foo in ref {i} times");
    }
}
fn deref_example() {
    let s1 = String::from("hello");
    foo(&s1);
    let count = Rc::new(s1);
    foo(&count);
    let f = &&Foo;
    f.foo(0);
    (&f).foo(1);
    (&&f).foo(2);
    (&&&&&f).foo(5);
}

pub fn practice() {
    example_box();
    example_leak_of_box();
    example_mybox();
    deref_example();
}
