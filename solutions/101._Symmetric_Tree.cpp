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
  public:
    bool isSymmetric(TreeNode *root) {
        if (!root) return true;
        if (!root->left && !root->right) return true;
        if ((root->left->right->val == root->right->left->val) && (root->left->left->val == root->right->right->val)) return true;
        return isSymmetric(root->left) && isSymmetric(root->right);
    }
};

class Solution {
  private:
    bool check_subtrees(TreeNode *left_subtree, TreeNode *right_subtree) {

        if (!left_subtree && !right_subtree) return true;
        if (!left_subtree || !right_subtree) return false;

        return (left_subtree->val == right_subtree->val) &&
            check_subtrees(left_subtree->left, right_subtree->right) &&
            check_subtrees(left_subtree->right, right_subtree->left);
    }

  public:
    bool isSymmetric(TreeNode *root) {
        if (!root) return true;
        return check_subtrees(root->left, root->right);
    }
};
