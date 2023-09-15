use crate::smart_pointer::reference_cycle::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

/// 运行结果如下
/// ```txt
/// a initial rc count = 1
/// a next item = Some(RefCell { value: Nil })
/// after b create a rc count = 2
/// b init rc count = 1
/// b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
/// after changing a: b rc count = 2
/// after changing a: a rc count = 2
/// ```
fn create_reference_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("after b create a rc count = {}", Rc::strong_count(&a));
    println!("b init rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("after changing a: b rc count = {}", Rc::strong_count(&b));
    println!("after changing a: a rc count = {}", Rc::strong_count(&a));
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // println!("a next item = {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

/// 运行结果如下
/// ```txt
/// leaf Rc strong_count = 1, weak = 0
/// leaf parent = None
/// baranch strong_count = 1, weak = 1
/// leaf after branch Rc strong_count = 2, weak = 0
/// leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) }, children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) }, children: RefCell { value: [] } }] } })
/// leaf after borrow Rc strong_count = 1, weak = 0
///
fn weak_to_avoid_reference_cycle() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf Rc strong_count = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "baranch strong_count = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf after branch Rc strong_count = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
    println!(
        "leaf after borrow Rc strong_count = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

pub fn reference_cycle_study() {
    create_reference_cycle();
    weak_to_avoid_reference_cycle();
}
