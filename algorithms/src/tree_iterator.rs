use algorithms_utils::tree;
use algorithms_utils::tree::TreeLink;
use algorithms_utils::tree::TreeNode;
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;

/// 前序遍历
/// 根->左->右
pub fn pre_order_iterator<T>(tree: &TreeNode<T>) -> RefCell<Vec<T>>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = RefCell::new(vec![]);
    result.borrow_mut().push(tree.val.clone());
    if let Some(left) = tree.left.clone() {
        result
            .borrow_mut()
            .append(pre_order_iterator(left.borrow().deref()).get_mut());
    }
    if let Some(right) = tree.right.clone() {
        result
            .borrow_mut()
            .append(pre_order_iterator(right.borrow().deref()).get_mut());
    }
    return result;
}

pub fn in_order_iterator<T>(tree: &TreeNode<T>) -> RefCell<Vec<T>>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = RefCell::new(vec![]);
    if let Some(left) = tree.left.clone() {
        result
            .borrow_mut()
            .append(in_order_iterator(left.borrow().deref()).get_mut());
    }
    result.borrow_mut().push(tree.val.clone());
    if let Some(right) = tree.right.clone() {
        result
            .borrow_mut()
            .append(in_order_iterator(right.borrow().deref()).get_mut());
    }
    return result;
}

pub fn post_order_iterator<T>(tree: &TreeNode<T>) -> RefCell<Vec<T>>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = RefCell::new(vec![]);
    if let Some(left) = tree.left.clone() {
        result
            .borrow_mut()
            .append(post_order_iterator(left.borrow().deref()).get_mut());
    }
    if let Some(right) = tree.right.clone() {
        result
            .borrow_mut()
            .append(post_order_iterator(right.borrow().deref()).get_mut());
    }
    result.borrow_mut().push(tree.val.clone());
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    use algorithms_utils::tree;
    use algorithms_utils::tree::TreeMaker;

    #[test]
    fn test_pre_order() {
        let tree: TreeLink<i32> =
            tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
        let result = pre_order_iterator(tree.unwrap().borrow().deref());
        println!("result: {:?}", result.borrow());
        assert_eq!(result.into_inner(), vec![8, 5, 1, 7, 10, 12]);
    }

    #[test]
    fn test_in_order() {
        let tree: TreeLink<i32> =
            tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
        let result = in_order_iterator(tree.unwrap().borrow().deref());
        println!("result: {:?}", result.borrow());
        assert_eq!(result.into_inner(), vec![1, 5, 7, 8, 10, 12]);
    }

    #[test]
    fn test_post_order() {
        let tree: TreeLink<i32> =
            tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
        let result = post_order_iterator(tree.unwrap().borrow().deref());
        println!("result: {:?}", result.borrow());
        assert_eq!(result.into_inner(), vec![1, 7, 5, 12, 10, 8]);
    }
}
