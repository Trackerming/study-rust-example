// 添加一个 use 语句将 Rc<T> 纳入范围，因为它不在序言中
use super::box_pointer::List;
use std::rc::Rc;

fn multi_pointer_one_box_data() {
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let b = List::Cons(15, Box::new(a));
    // error[E0382]: use of moved value: `a`
    // let c = List::Cons(18, Box::new(a));
}

#[derive(Debug)]
pub enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

/// 运行结果如下
/// ```txt
/// crate a: Rc::strong_count(&a) = 1
/// crate b and after clone a: Rc::strong_count(&a) = 2
/// crate c and after clone a: Rc::strong_count(&a) = 3
/// multi_pointer_one_rc_data b: Cons(15, Cons(5, Cons(10, Nil)))
/// multi_pointer_one_rc_data c: Cons(18, Cons(5, Cons(10, Nil)))
/// crate d and after clone d: Rc::strong_count(&a) = 4
/// leving d scope: Rc::strong_count(&a) = 3
/// ```
fn multi_pointer_one_rc_counter() {
    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("crate a: Rc::strong_count(&a) = {}", Rc::strong_count(&a));
    // Rc::clone 的实现不会像大多数类型的 clone 实现那样对所有数据进行深层复制。对 Rc::clone 的调用只会增加引用计数，这不会花费太多时间。数据的深层复制可能会花费大量时间
    let b = RcList::Cons(15, Rc::clone(&a));
    println!(
        "crate b and after clone a: Rc::strong_count(&a) = {}",
        Rc::strong_count(&a)
    );
    let c = RcList::Cons(18, Rc::clone(&a));
    println!(
        "crate c and after clone a: Rc::strong_count(&a) = {}",
        Rc::strong_count(&a)
    );
    println!("multi_pointer_one_rc_data b: {:?}", b);
    println!("multi_pointer_one_rc_data c: {:?}", c);
    {
        let d = RcList::Cons(21, Rc::clone(&a));
        println!(
            "crate d and after clone d: Rc::strong_count(&a) = {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "leving d scope: Rc::strong_count(&a) = {}",
        Rc::strong_count(&a)
    );
}

pub fn rc_t_count_pointer_study() {
    multi_pointer_one_box_data();
    multi_pointer_one_rc_counter();
}
