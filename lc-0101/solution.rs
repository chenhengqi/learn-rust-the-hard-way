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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let root = root.borrow();
                Self::is_symmetric_tree(root.left.clone(), root.right.clone())
            },
        }
    }

    fn is_symmetric_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, q) => q.is_none(),
            (p, None) => p.is_none(),
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    return false;
                }
                if !Self::is_symmetric_tree(p.borrow().left.clone(), q.borrow().right.clone()) {
                    return false;
                }
                Self::is_symmetric_tree(p.borrow().right.clone(), q.borrow().left.clone())
            },
        }
    }
}
