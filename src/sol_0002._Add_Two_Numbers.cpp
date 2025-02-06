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
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
        auto carry = 0;
        auto *dummy = new ListNode();
        auto *head = dummy;

        while (l1 != nullptr || l2 != nullptr || carry > 0) {
            int first_val = (l1 != nullptr) ? l1->val : 0;
            int sec_val = (l2 != nullptr) ? l2->val : 0;

            int value = first_val + sec_val + carry;
            carry = value / 10;

            head->next = new ListNode(value % 10);
            head = head->next;

            if (l1 != nullptr) l1 = l1->next;
            if (l2 != nullptr) l2 = l2->next;
        }

        return dummy->next;
    }
};