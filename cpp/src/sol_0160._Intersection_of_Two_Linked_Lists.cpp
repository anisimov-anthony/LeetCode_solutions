struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x)
        : val(x), next(nullptr) {}
};

#include <unordered_set>
class Solution {
  public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        // trivial case
        if (headA == nullptr || headB == nullptr) {
            return nullptr;
        }

        // main case
        /*
        // bad solution
        auto a_ptr = headA;
        while (a_ptr != nullptr) {
            auto b_ptr = headB;
            while (b_ptr != nullptr) {
                if (a_ptr == b_ptr) {
                    return a_ptr;
                }
                b_ptr = b_ptr->next;
            }
            a_ptr = a_ptr->next;
        }
        return nullptr;
        */

        // better solution
        std::unordered_set<ListNode *> st;
        while (headA != nullptr) {
            st.insert(headA);
            headA = headA->next;
        }

        while (headB != nullptr) {
            if (st.find(headB) != st.end()) {
                return headB;
            }
            headB = headB->next;
        }
        return nullptr;
    }
};