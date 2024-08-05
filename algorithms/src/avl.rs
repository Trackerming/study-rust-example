use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct TreeNode<T: Clone + Ord + Sized + Debug + PartialEq> {
    val: T,
    height: i32,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Clone + Ord + Sized + Debug + PartialEq> TreeNode<T> {
    fn new(val: T) -> Rc<RefCell<TreeNode<T>>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            height: 0,
            left: None,
            right: None,
        }))
    }
}

type OptionTreeNodeRc<T: Clone + Ord + Sized + Debug + PartialEq> =
    Option<Rc<RefCell<TreeNode<T>>>>;

struct AvlTree<T: Clone + Ord + Sized + Debug + PartialEq> {
    root: OptionTreeNodeRc<T>,
}

impl<T: Clone + Ord + Sized + Debug + PartialEq> AvlTree<T> {
    fn new() -> AvlTree<T> {
        Self { root: None }
    }

    // 返回高度
    fn height(node: OptionTreeNodeRc<T>) -> i32 {
        match node {
            None => -1,
            Some(node) => node.borrow().height,
        }
    }

    // 更新高度
    fn update_height(node: OptionTreeNodeRc<T>) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = std::cmp::max(Self::height(left), Self::height(right)) + 1;
        }
    }

    // 平衡因子
    fn balance_factor(node: OptionTreeNodeRc<T>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
        }
    }

    fn right_rotate(node: OptionTreeNodeRc<T>) -> OptionTreeNodeRc<T> {
        match node {
            Some(node) => {
                let child = node.borrow().left.clone().unwrap();
                let grand_child = child.borrow().right.clone();
                // 以child为原点，将node向右旋转
                child.borrow_mut().right = Some(node.clone());
                node.borrow_mut().left = grand_child;
                // 更新节点高度
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                // 返回旋转后的根节点
                Some(child)
            }
            None => None,
        }
    }

    fn left_rotate(node: OptionTreeNodeRc<T>) -> OptionTreeNodeRc<T> {
        match node {
            Some(node) => {
                let child = node.borrow().right.clone().unwrap();
                let grand_child = child.borrow().left.clone();
                // 以child为原点进行左旋
                child.borrow_mut().left = Some(node.clone());
                node.borrow_mut().right = grand_child;
                // 更新高度
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));
                Some(child)
            }
            None => None,
        }
    }

    fn rotate(node: OptionTreeNodeRc<T>) -> OptionTreeNodeRc<T> {
        // 获取节点的平衡因子
        let balance = Self::balance_factor(node.clone());
        // 左偏树
        if balance > 1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                let left = node.borrow().left.clone();
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
        }
        // 右偏树
        else if balance < -1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().right.clone()) <= 0 {
                Self::left_rotate(Some(node))
            } else {
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        }
        // 平衡树，无需旋转
        else {
            node
        }
    }

    fn insert_helper(node: OptionTreeNodeRc<T>, val: T) -> OptionTreeNodeRc<T> {
        match node {
            Some(mut node) => {
                /* 1. 查找插入位置并插入节点 */
                match {
                    let node_val = node.borrow().val.clone();
                    node_val
                }
                .cmp(&val)
                {
                    std::cmp::Ordering::Greater => {
                        let left = node.borrow().left.clone();
                        node.borrow_mut().left = Self::insert_helper(left, val);
                    }
                    std::cmp::Ordering::Less => {
                        let right = node.borrow().right.clone();
                        node.borrow_mut().right = Self::insert_helper(right, val);
                    }
                    std::cmp::Ordering::Equal => {
                        return Some(node); // 重复节点不插入，直接返回
                    }
                }
                Self::update_height(Some(node.clone())); // 更新节点高度

                /* 2. 执行旋转操作，使该子树重新恢复平衡 */
                node = Self::rotate(Some(node)).unwrap();
                // 返回子树的根节点
                Some(node)
            }
            None => Some(TreeNode::new(val)),
        }
    }

    fn insert(&mut self, val: T) {
        self.root = Self::insert_helper(self.root.clone(), val);
    }

    fn remove_helper(node: OptionTreeNodeRc<T>, val: T) -> OptionTreeNodeRc<T> {
        match node {
            Some(mut node) => {
                if val < node.borrow().val {
                    let left = node.borrow().left.clone();
                    node.borrow_mut().left = Self::remove_helper(left, val);
                } else if val > node.borrow().val {
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::remove_helper(right, val);
                } else if node.borrow().left.is_none() || node.borrow().right.is_none() {
                    let child = if node.borrow().left.is_some() {
                        node.borrow().left.clone()
                    } else {
                        node.borrow().right.clone()
                    };
                    match child {
                        // 子节点数量 = 0 ，直接删除 node 并返回
                        None => {
                            return None;
                        }
                        // 子节点数量 = 1 ，直接删除 node
                        Some(child) => node = child,
                    }
                } else {
                    // 子节点数量 = 2 ，则将中序遍历的下个节点删除，并用该节点替换当前节点
                    let mut temp = node.borrow().right.clone().unwrap();
                    loop {
                        let temp_left = temp.borrow().left.clone();
                        if temp_left.is_none() {
                            break;
                        }
                        temp = temp_left.unwrap();
                    }
                    let right = node.borrow().right.clone();
                    node.borrow_mut().right = Self::remove_helper(right, temp.borrow().val.clone());
                    node.borrow_mut().val = temp.borrow().val.clone();
                }
                Self::update_height(Some(node.clone()));
                /* 2. 执行旋转操作，使该子树重新恢复平衡 */
                node = Self::rotate(Some(node)).unwrap();
                // 返回子树的根节点
                Some(node)
            }
            None => None,
        }
    }

    fn remove(&mut self, val: T) {
        Self::remove_helper(self.root.clone(), val);
    }

    fn search(&self, val: T) -> OptionTreeNodeRc<T> {
        let mut cur = self.root.clone();
        while let Some(current) = cur.clone() {
            match current.borrow().val.cmp(&val) {
                std::cmp::Ordering::Less => {
                    cur = current.borrow().right.clone();
                }
                std::cmp::Ordering::Greater => {
                    cur = current.borrow().left.clone();
                }
                std::cmp::Ordering::Equal => {
                    break;
                }
            }
        }
        cur
    }
}

#[cfg(test)]
mod test_avl {
    use super::*;

    #[test]
    fn test_op() {
        let mut avl_tree = AvlTree::new();
        for i in -1..10 {
            avl_tree.insert(i);
        }
        // 插入重复节点
        avl_tree.insert(5);
        avl_tree.remove(8);
        avl_tree.remove(3);

        /* 查询节点 */
        let node = avl_tree.search(7);
        if let Some(node) = node.clone() {
            println!(
                "\n查找到的节点对象为 {:?}，节点值 = {:?}",
                &*node.borrow(),
                node.borrow().val
            );
        }
        assert_eq!(node, Some(TreeNode::new(7)));
    }
}
