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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            None => false,
            Some(root) => {
                let root = root.borrow();
                if root.left.is_none() && root.right.is_none() {
                    return root.val == sum;
                }
                let sum = sum - root.val;
                Self::has_path_sum(root.left.clone(), sum) || Self::has_path_sum(root.right.clone(), sum)
            }
        }
    }
}
