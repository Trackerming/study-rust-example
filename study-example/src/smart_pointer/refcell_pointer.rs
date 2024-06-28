use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessager {
    sent_message: std::cell::RefCell<Vec<String>>,
}

impl MockMessager {
    fn new() -> MockMessager {
        MockMessager {
            sent_message: std::cell::RefCell::new(vec![]),
        }
    }
}

impl Messager for MockMessager {
    fn send(&self, msg: &str) {
        // error[E0596]: cannot borrow `self.sent_message` as mutable, as it is behind a `&` reference
        // 解法：采用refcell类型的指针
        self.sent_message.borrow_mut().push(String::from(msg));
        self.sent_message.borrow_mut().push(String::from(msg));
        let mut borrow1 = self.sent_message.borrow_mut();
        // 运行时错误： thread 'main' panicked at 'already borrowed: BorrowMutError', study-example/bin/smart_pointer/refcell_pointer.rs:57:45
        // let mut borrow2 = self.sent_message.borrow_mut();
        borrow1.push(String::from(msg));
        // borrow2.push(String::from(msg));
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
/// 组合Rc<T>和RcCell<T>
fn cons_modify_usage() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    let mut value_borrow = (*value).borrow_mut();
    *value_borrow += 100;
    println!("value: {:?}", value);
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
/// 调用结果如下
/// ```txt
/// mock message len: 3
/// a after = Cons(RefCell { value: <borrowed> }, Nil)
/// b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: <borrowed> }, Nil))
/// c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: <borrowed> }, Nil))
/// ```
pub fn refcell_pointer_study() {
    let mock_messager = MockMessager::new();
    let mut limit_tracker = LimitTracker::new(&mock_messager, 100);
    limit_tracker.set_value(80);
    println!(
        "mock message len: {}",
        mock_messager.sent_message.borrow().len()
    );
    cons_modify_usage();
}
