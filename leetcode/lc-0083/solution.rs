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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => head,
            Some(mut head) => {
                if head.next.is_none() {
                    return Some(head);
                }
                let next = head.next.unwrap();
                if head.val != next.val {
                    let new_head = Self::delete_duplicates(Some(next));
                    head.next = new_head;
                    Some(head)
                } else {
                    head.next = next.next;
                    Self::delete_duplicates(Some(head))
                }
            }
        }
    }
}
