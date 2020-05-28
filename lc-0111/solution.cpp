/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    int minDepth(TreeNode* root) {
        int depth = INT_MAX;
        int currentDepth = 0;
        minDepth(root, currentDepth, depth);
        return depth;
    }

    int minDepth(TreeNode* root, int currentDepth, int& depth) {
        if (!root) {
            depth = min(depth, currentDepth);
            return currentDepth;
        }
        if (!root->left && !root->right) {
            depth = min(depth, currentDepth + 1);
            return currentDepth + 1;
        }
        int left = INT_MAX;
        if (root->left) left = minDepth(root->left, currentDepth + 1, depth);
        int right = INT_MAX;
        if (root->right) right = minDepth(root->right, currentDepth + 1, depth);
        return min(left, right);
    }
};
