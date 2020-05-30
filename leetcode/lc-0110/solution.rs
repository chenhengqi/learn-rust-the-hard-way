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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let root = root.borrow();
                (Self::get_height(root.left.clone()) - Self::get_height(root.right.clone())).abs() <= 1 && Self::is_balanced(root.left.clone()) && Self::is_balanced(root.right.clone())
            },
        }
    }

    fn get_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let root = root.borrow();
                1 + std::cmp::max(Self::get_height(root.left.clone()), Self::get_height(root.right.clone()))
            }
        }
    }
}
