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
  private:
    int get_length(ListNode *head) {
        int length = 0;
        while (head != nullptr) {
            ++length;
            head = head->next;
        }
        return length;
    }

  public:
    ListNode *rotateRight(ListNode *head, int k) {
        // corner cases
        if (head == nullptr || head->next == nullptr || k == 0) return head;

        int len = get_length(head);
        int breaking_point = k % len;

        if (breaking_point == 0) return head;

        // main case
        ListNode *prev = head;
        for (int i = 0; i < len - breaking_point - 1; ++i) {
            prev = prev->next;
        }

        ListNode *new_head = prev->next;
        prev->next = nullptr;

        ListNode *new_tail = new_head;
        while (new_tail->next != nullptr) {
            new_tail = new_tail->next;
        }

        new_tail->next = head;

        return new_head;
    }
};
