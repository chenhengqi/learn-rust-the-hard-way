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
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        if (!l1 && !l2) return nullptr;
        if (!l1) return l2;
        if (!l2) return l1;
        if (l1->val <= l2->val) {
            auto l = mergeTwoLists(l1->next, l2);
            l1->next = l;
            return l1;
        } else {
            auto l = mergeTwoLists(l2->next, l1);
            l2->next = l;
            return l2;
        }
    }
};
