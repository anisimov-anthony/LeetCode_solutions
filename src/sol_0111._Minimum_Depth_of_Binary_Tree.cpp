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

// difference between maxDepth and minDepth is in std::min/max
#include <algorithm>
class Solution {
  public:
    int minDepth(TreeNode *root) {

        // corner-cases
        if (!root) return 0;
        if (!root->left && !root->right) return 1;

        // skip null-paths
        if (!root->left) return minDepth(root->right) + 1;
        if (!root->right) return minDepth(root->left) + 1;

        // general case
        return std::min(minDepth(root->left), minDepth(root->right)) + 1;
    }
};