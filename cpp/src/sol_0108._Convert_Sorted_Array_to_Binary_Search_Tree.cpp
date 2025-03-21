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
    TreeNode *sortedArrayToBST(std::vector<int> &nums) {
        return sortedArrayToBST(nums.begin(), nums.end());
    }

  private:
    TreeNode *sortedArrayToBST(std::vector<int>::iterator left, std::vector<int>::iterator right) {
        if (left >= right) return nullptr;
        auto mid = left + (right - left) / 2;
        TreeNode *root = new TreeNode(*mid);
        root->left = sortedArrayToBST(left, mid);
        root->right = sortedArrayToBST(mid + 1, right);
        return root;
    }
};