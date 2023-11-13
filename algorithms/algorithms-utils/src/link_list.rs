use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkNode<T> {
    pub value: T,
    pub next: Link<T>,
}

pub type Link<T> = Option<Rc<RefCell<LinkNode<T>>>>;

pub struct LinkList<T> {
    pub head: Link<T>,
}

#[macro_export]
macro_rules! link_list {
    ($e: expr, $n: expr) => {
        LinkList::link_maker($e, $n)
    };
}

pub trait LinkMaker<T> {
    fn link_maker(val: T, next: Link<T>) -> Link<T> {
        Some(Rc::new(RefCell::new(LinkNode { value: val, next })))
    }
}

impl<T> LinkMaker<T> for LinkList<T> {}

pub struct Iter<T> {
    next: Link<T>,
}

impl<T> LinkList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.clone(),
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(node) => {
                self.next = node.borrow().next.clone();
                Some(node.borrow().value.clone())
            }
            None => None,
        }
    }
}
