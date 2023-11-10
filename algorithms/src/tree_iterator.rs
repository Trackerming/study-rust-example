use algorithms_utils::tree;
use algorithms_utils::tree::TreeLink;
use algorithms_utils::tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

/// 前序遍历
/// 根->左->右
pub fn pre_order_traversal<T>(tree: &TreeNode<T>) -> RefCell<Vec<T>>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = RefCell::new(vec![]);
    result.borrow_mut().push(tree.val.clone());
    if let Some(left) = tree.left.clone() {
        result
            .borrow_mut()
            .append(pre_order_traversal(left.borrow().deref()).get_mut());
    }
    if let Some(right) = tree.right.clone() {
        result
            .borrow_mut()
            .append(pre_order_traversal(right.borrow().deref()).get_mut());
    }
    return result;
}

pub fn in_order_traversal<T>(tree: &TreeNode<T>) -> RefCell<Vec<T>>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = RefCell::new(vec![]);
    if let Some(left) = tree.left.clone() {
        result
            .borrow_mut()
            .append(in_order_traversal(left.borrow().deref()).get_mut());
    }
    result.borrow_mut().push(tree.val.clone());
    if let Some(right) = tree.right.clone() {
        result
            .borrow_mut()
            .append(in_order_traversal(right.borrow().deref()).get_mut());
    }
    return result;
}

pub fn post_order_traversal<T>(tree: &TreeNode<T>) -> RefCell<Vec<T>>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = RefCell::new(vec![]);
    if let Some(left) = tree.left.clone() {
        result
            .borrow_mut()
            .append(post_order_traversal(left.borrow().deref()).get_mut());
    }
    if let Some(right) = tree.right.clone() {
        result
            .borrow_mut()
            .append(post_order_traversal(right.borrow().deref()).get_mut());
    }
    result.borrow_mut().push(tree.val.clone());
    return result;
}

/// 采用迭代的方式进行，用到栈的操作，控制入栈和出栈顺序
pub fn pre_order_iterator<T>(tree: &TreeNode<T>) -> Vec<T>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = vec![];
    let mut stack = VecDeque::new();
    stack.push_back((*tree).clone());
    while !stack.is_empty() {
        if let Some(node) = stack.pop_back() {
            result.push(node.val.clone());
            if let Some(right) = node.right.clone() {
                stack.push_back((*right.borrow().deref()).clone());
            }
            if let Some(left) = node.left.clone() {
                stack.push_back((*left.borrow().deref()).clone());
            }
        }
    }
    return result;
}

pub fn in_order_iterator<T>(tree: &TreeNode<T>) -> Vec<T>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = vec![];
    let mut stack = VecDeque::new();
    let mut current_node = Some(Rc::new(RefCell::new(tree.clone())));
    while !stack.is_empty() || current_node != None {
        match current_node.clone() {
            Some(node) => {
                stack.push_back(Some(node.clone()));
                current_node = node.borrow().left.clone();
            }
            None => {
                let current_node = stack.pop_back().unwrap();
                if let Some(node) = current_node.clone() {
                    result.push(node.borrow().val.clone());
                    stack.push_back(node.borrow().right.clone());
                }
            }
        }
    }
    return result;
}

/// 跟前序遍历差不多，后序遍历为 左 -> 右 -> 根
/// 逆序就变成了 根 -> 右 -> 左
pub fn post_order_iterator<T>(tree: &TreeNode<T>) -> Vec<T>
where
    T: Debug + Ord + Eq + PartialOrd + PartialEq + Clone,
{
    let mut result = vec![];
    let mut stack = VecDeque::new();
    stack.push_back(tree.clone());
    while !stack.is_empty() {
        if let Some(node) = stack.pop_back() {
            if let Some(left) = node.left {
                stack.push_back(left.borrow().clone());
            }
            if let Some(right) = node.right {
                stack.push_back(right.borrow().clone());
            }
            result.push(node.val.clone());
        }
    }
    result.reverse();
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
        let result = pre_order_traversal(tree.clone().unwrap().borrow().deref());
        println!("result: {:?}", result.borrow());
        assert_eq!(result.into_inner(), vec![8, 5, 1, 7, 10, 12]);
        let result = pre_order_iterator(tree.unwrap().borrow().deref());
        assert_eq!(result, vec![8, 5, 1, 7, 10, 12]);
    }

    #[test]
    fn test_in_order() {
        let tree: TreeLink<i32> =
            tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
        let result = in_order_traversal(tree.clone().unwrap().borrow().deref());
        println!("result: {:?}", result.borrow());
        assert_eq!(result.into_inner(), vec![1, 5, 7, 8, 10, 12]);
        let result = in_order_iterator(tree.unwrap().borrow().deref());
        println!("result: {:?}", result);
        assert_eq!(result, vec![1, 5, 7, 8, 10, 12]);
    }

    #[test]
    fn test_post_order() {
        let tree: TreeLink<i32> =
            tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
        let result = post_order_traversal(tree.clone().unwrap().borrow().deref());
        println!("result: {:?}", result.borrow());
        assert_eq!(result.into_inner(), vec![1, 7, 5, 12, 10, 8]);
        let result = post_order_iterator(tree.unwrap().borrow().deref());
        println!("result: {:?}", result);
        assert_eq!(result, vec![1, 7, 5, 12, 10, 8]);
    }
}
