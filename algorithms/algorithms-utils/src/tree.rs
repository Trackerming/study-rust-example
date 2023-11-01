use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: TreeLink<T>,
    pub right: TreeLink<T>,
}

pub type TreeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[macro_export]
macro_rules! tree {
    ($e:expr) => {
        TreeLink::leaf($e)
    };
    ($e:expr, $l:expr, $r:expr) => {
        TreeLink::branch($e, $l, $r)
    };
}

pub trait TreeMaker<T> {
    fn branch(val: T, left: TreeLink<T>, right: TreeLink<T>) -> TreeLink<T> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    fn leaf(val: T) -> TreeLink<T> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl<T> TreeMaker<T> for TreeLink<T> {}
