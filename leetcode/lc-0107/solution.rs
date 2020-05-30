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
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut result = vec![];
        let mut nodes = VecDeque::new();
        nodes.push_back(root.clone());
        while !nodes.is_empty() {
            let mut level = vec![];
            let n = nodes.len();
            for i in 0..n {
                let front = nodes.pop_front().unwrap();
                let root = front.unwrap();
                let node = root.borrow();
                level.push(node.val);
                if node.left.is_some() {
                    nodes.push_back(node.left.clone());
                }
                if node.right.is_some() {
                    nodes.push_back(node.right.clone());
                }
            }
            result.push(level);
        }
        result.reverse();
        result
    }
}
