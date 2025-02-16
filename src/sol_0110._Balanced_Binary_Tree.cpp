#include <algorithm>

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

class Solution {
  private:
    int height(TreeNode *root) {
        // corner case
        if (!root) return 0;

        int left_height = height(root->left);
        int right_height = height(root->right);

        if (left_height == -1 || right_height == -1) return -1;

        // main case
        if (std::abs(left_height - right_height) > 1) return -1;

        return 1 + std::max(left_height, right_height);
    }

  public:
    bool isBalanced(TreeNode *root) {
        return height(root) != -1;
    }
};