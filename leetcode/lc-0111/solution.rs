// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_depth = std::i32::MAX;
        let current_depth = 0;
        Self::tree_depth(root.clone(), current_depth, &mut min_depth);
        min_depth
    }

    fn tree_depth(root: Option<Rc<RefCell<TreeNode>>>, current_depth: i32, min_depth: &mut i32) -> i32 {
        match root {
            None => {
                *min_depth = std::cmp::min(*min_depth, current_depth);
                current_depth
            },
            Some(root) => {
                let root = root.borrow();
                if root.left.is_none() && root.right.is_none() {
                    *min_depth = std::cmp::min(*min_depth, current_depth + 1);
                    return current_depth + 1;
                }
                let mut left_depth = std::i32::MAX;
                if root.left.is_some() {
                    left_depth = Self::tree_depth(root.left.clone(), current_depth + 1, min_depth);
                }
                let mut right_depth = std::i32::MAX;
                if root.right.is_some() {
                    right_depth = Self::tree_depth(root.right.clone(), current_depth + 1, min_depth);
                }
                std::cmp::min(left_depth, right_depth)
            },
        }
    }
}
