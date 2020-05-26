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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree(nums: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if l > r {
                return None;
            }
            let m = l + (r - l) / 2;
            let mut root = TreeNode::new(nums[m as usize]);
            root.left = build_tree(nums, l, m - 1);
            root.right = build_tree(nums, m + 1, r);
            Some(Rc::new(RefCell::new(root)))
        }
        if nums.is_empty() {
            return None;
        }
        build_tree(&nums, 0, (nums.len() - 1) as i32)
    }
}
