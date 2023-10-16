/// 循环引用
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

// 创建两个互相指向的reference
fn loop_ref_example() {
    let a = Rc::new(List::Cons(32, RefCell::new(Rc::new(List::Nil))));
    println!("a的初始化Rc计数 = {}", Rc::strong_count(&a));
    println!("init a指向节点： {:?}", a.tail());
    // 创建b指向a；
    let b = Rc::new(List::Cons(64, RefCell::new(Rc::clone(&a))));
    println!("创建指向a的b之后a的Rc计数 = {}", Rc::strong_count(&a));
    println!("创建指向a的b之后b的Rc计数 = {}", Rc::strong_count(&b));
    println!("b指向节点： {:?}", b.tail());
    // 修改a指向b
    if let Some(a_next) = a.tail() {
        *a_next.borrow_mut() = Rc::clone(&b);
    }
    println!("修改a指向b之后a的Rc计数 = {}", Rc::strong_count(&a));
    println!("修改a指向b之后b的Rc计数 = {}", Rc::strong_count(&b));
    // println!("a指向节点: {:?}", a.tail());
    // println!("b指向节点: {:?}", b.tail());
}

/// Rc -> downgrade成 -> Weak
/// Weak -> uprgade成 -> Rc
/// weak可访问但是无所有权，不增加引用计数，不影响值的释放回收
use std::rc::Weak;

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

// 循环引用示例
fn fix_loop_ref() {
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });
    // 工具1 属于 owner
    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    });
    // 工具2 属于 owner
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    });
    // owner拥有工具1和2，使用Rc就会被循环引用，所以降级为Weak
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget1));
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget2));
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}

fn tree_example() {
    let leaf = Rc::new(Node {
        value: 8,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(Vec::new()),
    });
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 32,
            parent: RefCell::new(Weak::new()),
            child: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

pub fn practice() {
    loop_ref_example();
    fix_loop_ref();
    tree_example();
}
