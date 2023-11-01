use algorithms_utils::tree;
use algorithms_utils::tree::TreeLink;
use algorithms_utils::tree::TreeMaker;

// https://github.com/warycat/rustgym/blob/master/leetcode/src/d10/_1008_construct_binary_search_tree_from_preorder_traversal.rs

struct Solution;

trait Postorder<T: std::cmp::Ord + Copy> {
    fn from_vec(preorder: &[T], inorder: &[T]) -> Self;
}

impl<T: std::cmp::Ord + Copy> Postorder<T> for TreeLink<T> {
    fn from_vec(preorder: &[T], inorder: &[T]) -> Self {
        let n = preorder.len();
        if n == 0 {
            None
        } else {
            if n == 1 {
                tree!(preorder[0])
            } else {
                let i = inorder.binary_search(&preorder[0]).unwrap();
                tree!(
                    preorder[0],
                    TreeLink::from_vec(&preorder[1..=i], &inorder[0..i]),
                    TreeLink::from_vec(&preorder[i + 1..], &inorder[i + 1..])
                )
            }
        }
    }
}

impl Solution {
    fn bst_from_preorder<T: std::cmp::Ord + Clone + Copy>(preorder: Vec<T>) -> TreeLink<T> {
        let mut inorder: Vec<T> = preorder.clone();
        inorder.sort_unstable();
        TreeLink::from_vec(&preorder, &inorder)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let preorder = vec![8, 5, 1, 7, 10, 12];
        let res = tree!(8, tree!(5, tree!(1), tree!(7)), tree!(10, None, tree!(12)));
        assert_eq!(Solution::bst_from_preorder(preorder), res);
    }
}
