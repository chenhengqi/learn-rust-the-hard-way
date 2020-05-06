// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (l1, None) => l1,
            (None, l2) => l2,
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    let l = Self::merge_two_lists(l1.next, Some(l2));
                    l1.next = l;
                    return Some(l1);
                } else {
                    let l = Self::merge_two_lists(l2.next, Some(l1));
                    l2.next = l;
                    return Some(l2);
                }
            },
        }
    }
}
