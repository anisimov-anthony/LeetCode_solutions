struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode()
        : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x)
        : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};

#include <algorithm>
class Solution {
  public:
    int maxDepth(TreeNode *root) {

        // corner-cases
        if (!root) return 0;
        if (!root->left && !root->right) return 1;

        // skip null-paths
        if (!root->left) return maxDepth(root->right) + 1;
        if (!root->right) return maxDepth(root->left) + 1;

        // general case
        return std::max(maxDepth(root->left), maxDepth(root->right)) + 1;
    }
};