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

/// Drop特征

/// RC和Arc
fn rc_example() {
    let a = Rc::new(String::from("rust"));
    // 这里的 clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
}
struct Owner {
    name: String,
}
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}
fn example_rc_all() {
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Rust".to_string(),
    });
    let gadget1 = Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget3 = Gadget {
        id: 3,
        owner: gadget_owner.clone(),
    };
    drop(gadget_owner);
    println!("Gadget count {}", Rc::strong_count(&gadget1.owner));
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
    println!("Gadget {} owned by {}", gadget3.id, gadget3.owner.name);
}
use std::sync::Arc;
use std::thread;
// Arc 线程安全的引用计数，与Rc拥有一样的API
fn arc_example() {
    let a_str = Arc::new(String::from("Arc example"));
    for i in 0..10 {
        let a_str_clone = Arc::clone(&a_str);
        let current_i = i;
        thread::spawn(move || {
            // let a_str_clone = a_str.clone();
            println!("another {current_i} thread {}", a_str_clone);
        });
    }
}

/// Cell和RefCell
use std::cell::Cell;
fn example_cell() {
    let c = Cell::new("rust cell");
    let one = c.get();
    c.set("rust cell edit.");
    let two = c.get();
    println!("one {}, two {}", one, two);
    let x = Cell::new(24);
    let y = &x;
    let z = &x;
    x.set(24 + 1);
    y.set(24 + 2);
    z.set(24 + 3);
    println!("x: {}, y: {}. z: {}", x.get(), y.get(), z.get());
}
use std::cell::RefCell;
// 将不可变引用转化为可变引用
trait Message {
    fn send(&self, msg: String);
}
#[derive(Debug)]
struct MessageQueue {
    msg_cache: RefCell<Vec<String>>,
}
impl Message for MessageQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}
fn refcell_example() {
    let message_queue = MessageQueue {
        msg_cache: RefCell::new(vec!["rust".to_string(), "refcell".to_string()]),
    };
    message_queue.send("hello".to_string());
    println!("message queue: {:?}", message_queue);
}

fn refcell_rc_mix() {
    let str0 = Rc::new(RefCell::new("multi owner of me".to_string()));
    let str_ref_clone1 = str0.clone();
    let str_ref_clone2 = Rc::clone(&str0);
    str_ref_clone2.borrow_mut().push_str("refcell_rc_mix");
    str_ref_clone1.borrow_mut().push_str(" 2 times");
    /*
    RefCell { value: "multi owner of merefcell_rc_mix 2 times" }
    RefCell { value: "multi owner of merefcell_rc_mix 2 times" }
    RefCell { value: "multi owner of merefcell_rc_mix 2 times" }
    */
    println!("{:?}\n{:?}\n{:?}", str0, str_ref_clone1, str_ref_clone2);
}

/// Cell::from_mut和as_slice_of_cells方法
fn is_even(i: i32) -> bool {
    i % 2 == 0
}
fn retain_even(nums: &mut Vec<i32>) {
    /*let mut i = 0;
    for num in nums.iter().filter(|&n| is_even(*n)) {
        nums[i] = *num;
        i += 1;
    }
    nums.truncate(i);*/
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    let mut i = 0;
    for num in slice.iter().filter(|n| is_even(n.get())) {
        slice[i].set(num.get());
        i += 1;
    }
    nums.truncate(i);
    println!("nums {:?}", nums);
}
pub fn practice() {
    example_box();
    example_leak_of_box();
    example_mybox();
    deref_example();
    rc_example();
    example_rc_all();
    arc_example();
    example_cell();
    refcell_example();
    refcell_rc_mix();
    let mut vec_val: Vec<i32> = vec![12, 34, 55, 66, 77, 99];
    retain_even(&mut vec_val);
}
