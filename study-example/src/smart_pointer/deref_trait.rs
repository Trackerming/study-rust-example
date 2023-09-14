fn pointer_to_int() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // error[E0277]: can't compare `{integer}` with `&{integer}`
    // assert_eq!(5, y);
    let z = Box::new(x);
    assert_eq!(5, *z);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    // type Target = T; 语法定义了要使用的 Deref 特征的关联类型
    // 联类型是声明泛型参数的一种稍微不同的方式
    type Target = T;

    // 如果没有 Deref 特征，编译器只能取消引用 & 引用。
    // deref 方法使编译器能够获取实现 Deref 的任何类型的值，并调用 deref 方法来获取 & 引用它知道如何取消引用
    fn deref(&self) -> &Self::Target {
        // .0 访问元组结构中的第一个值
        &self.0
    }
}

// 参数为字符串切片
fn hello_print(str_val: &str) {
    println!("hello {str_val}");
}

fn my_box_test() {
    let x = 10;
    let y = MyBox::new(x);
    assert_eq!(10, x);
    // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced，
    // 解决办法：在MyBox上实现Deref Trait
    assert_eq!(10, *y); // *y实际上等同于 *(y.deref());
    let string_type_box = MyBox::new(String::from("Rust deref coercion"));
    /*
       step 1: 因为MyBox实现了Deref trait， &string_type_box 调用 deref转换为了&String
       step 2: 因为String实现了Deref trait，Rust再次调用Deref将&String转换为了&str
    */
    hello_print(&string_type_box);
    // 等同于如下 *先deref拿到字符串 &取字符串的引用 [..]获取切片
    hello_print(&(*string_type_box)[..]);
}

pub fn deref_trait_study() {
    pointer_to_int();
    my_box_test();
}
