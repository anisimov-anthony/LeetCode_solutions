#include <vector>

class Solution {
  public:
    int returnToBoundaryCount(std::vector<int> &nums) {
        auto crosses = 0;
        auto position = 0;
        for (size_t i = 0; i < nums.size(); i++) {
            position = position + nums[i];
            if (position == 0) {
                ++crosses;
            }
        }

        return crosses;
    }
};