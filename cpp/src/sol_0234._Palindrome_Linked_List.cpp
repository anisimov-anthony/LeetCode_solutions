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

#include <deque>
class Solution {
  private:
    ListNode *reverseList(ListNode *head) {
        ListNode *prev = nullptr, *next = nullptr;
        while (head) {
            next = head->next;
            head->next = prev;
            prev = head;
            head = next;
        }
        return prev;
    };

  public:
    bool isPalindrome(ListNode *head) {
        // trivial case
        if (head == nullptr || head->next == nullptr) {
            return true;
        }

        // bad solution
        /*
        // main case
        std::deque<int> st;
        while (head != nullptr) {
            st.push_back(head->val);
            head = head->next;
        }

        while (st.size() > 1) {
            if (st.front() != st.back()) {
                return false;
            }
            st.pop_front();
            st.pop_back();
        }

        return true;
        */

        // better solution
        auto slow = head;
        auto fast = head;
        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
        }

        ListNode *revHead = reverseList(slow);

        ListNode *p1 = head;
        ListNode *p2 = revHead;
        while (p2 != nullptr) {
            if (p1->val != p2->val) return false;
            p1 = p1->next;
            p2 = p2->next;
        }

        return true;
    }
};