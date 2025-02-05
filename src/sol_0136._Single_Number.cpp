#include <vector>

class Solution {
  public:
    int singleNumber(std::vector<int> &nums) {
        auto single = 0;
        for (auto i : nums) {
            single ^= i;
        }
        return single;
    }
};