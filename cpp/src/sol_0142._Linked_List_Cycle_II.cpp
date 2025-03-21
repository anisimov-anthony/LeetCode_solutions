struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x)
        : val(x), next(nullptr) {}
};

#include <unordered_set>

class Solution {
  public:
    ListNode *detectCycle(ListNode *head) {
        // trivial case
        if (head == nullptr || head->next == nullptr) {
            return nullptr;
        }

        std::unordered_set<ListNode *> st;

        while (head->next != nullptr) {
            if (st.find(head) != st.end()) {
                return head;
            }

            st.insert(head);
            head = head->next;
        }
        return nullptr;
    }
};