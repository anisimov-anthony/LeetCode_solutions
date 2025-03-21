#include <queue>
#include <vector>

class Solution {
  public:
    int timeRequiredToBuy(std::vector<int> &tickets, int k) {
        std::queue<std::pair<int, int>> q;

        for (size_t i = 0; i < tickets.size(); i++) {
            q.push({tickets[i], i});
        }

        int timer = 0;

        while (!q.empty()) {
            auto front = q.front();
            q.pop();

            front.first--;
            timer++;

            if (front.second == k && front.first == 0) {
                break;
            }

            if (front.first > 0) {
                q.push(front);
            }
        }

        return timer;
    }
};
