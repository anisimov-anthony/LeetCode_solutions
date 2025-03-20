#include <algorithm>
#include <string>
#include <vector>

class Solution {
  public:
    bool isAlienSorted(std::vector<std::string> &words, std::string order) {

        int char_order[26];
        for (int i = 0; i < order.size(); ++i) {
            char_order[order[i] - 'a'] = i;
        }

        std::vector<std::string> valid = words;
        auto comparator = [&](const std::string &a, const std::string &b) {
            for (size_t i = 0; i < a.size(); ++i) {
                if (i >= b.size()) {
                    return false;
                }

                int order_a = char_order[a[i] - 'a'];
                int order_b = char_order[b[i] - 'a'];

                if (order_a < order_b) {
                    return true;
                } else if (order_a > order_b) {
                    return false;
                }
            }
            return a.size() < b.size();
        };

        std::sort(valid.begin(), valid.end(), comparator);

        return valid == words;
    }
};