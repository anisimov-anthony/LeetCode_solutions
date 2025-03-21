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

#include <vector>
class Solution {
  public:
    std::vector<int> inorderTraversal(TreeNode *root) {
        std::vector<int> result;
        inorder(root, result);
        return result;
    }

  private:
    void inorder(TreeNode *node, std::vector<int> &result) {
        if (!node) return;
        inorder(node->left, result);
        result.push_back(node->val);
        inorder(node->right, result);
    }
};