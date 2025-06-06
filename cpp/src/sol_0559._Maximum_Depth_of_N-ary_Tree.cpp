#include <vector>

class Node {
  public:
    int val;
    std::vector<Node *> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, std::vector<Node *> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
  public:
    int maxDepth(Node *root) {

        // corner case
        if (root == nullptr) {
            return 0;
        }

        auto result = 0;
        auto next_layer = root->children;

        for (auto child : next_layer) {
            result = std::max(result, maxDepth(child));
        }

        return 1 + result;
    }
};
