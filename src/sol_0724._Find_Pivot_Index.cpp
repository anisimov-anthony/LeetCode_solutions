#include <numeric>
#include <vector>

class Solution {
  public:
    int pivotIndex(std::vector<int> &nums) {
        // trivial case
        if (nums.empty()) {
            return -1;
        }
        if (nums.size() == 1) {
            return 0;
        }

        // main case
        auto full_sum = std::accumulate(nums.begin(), nums.end(), 0);
        auto left_sum = 0;

        for (size_t i = 0; i < nums.size(); i++) {
            if (left_sum == full_sum - left_sum - nums[i]) {
                return i;
            }
            left_sum += nums[i];
        }
        return -1;
    }
};