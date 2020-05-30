/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        if (!head || !head->next) return head;
        if (head->val != head->next->val) {
            auto newHead = deleteDuplicates(head->next);
            head->next = newHead;
            return head;
        } else {
            head->next = head->next->next;
            return deleteDuplicates(head);
        }
    }
};
