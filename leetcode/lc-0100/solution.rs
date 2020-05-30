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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, q) => q.is_none(),
            (p, None) => p.is_none(),
            (Some(p), Some(q)) => {
                if p.borrow().val != q.borrow().val {
                    return false;
                }
                if !Self::is_same_tree(p.borrow_mut().left.clone(), q.borrow_mut().left.clone()) {
                    return false;
                }
                Self::is_same_tree(p.borrow_mut().right.clone(), q.borrow_mut().right.clone())
            },
        }
    }
}
