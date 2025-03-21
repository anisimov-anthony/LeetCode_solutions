
struct ListNode {
    int val;
    ListNode *next;
    ListNode()
        : val(0), next(nullptr) {}
    ListNode(int x)
        : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next)
        : val(x), next(next) {}
};

class Solution {
  public:
    ListNode *deleteDuplicates(ListNode *head) {
        if (head == nullptr) {
            return head;
        }

        auto current_list = head;

        while (current_list != nullptr && current_list->next != nullptr) {
            if (current_list->val == current_list->next->val) {
                auto top_new_chain = current_list->next->next;
                delete current_list->next;
                current_list->next = top_new_chain;
            } else {
                current_list = current_list->next;
            }
        }
        return head;
    }
};