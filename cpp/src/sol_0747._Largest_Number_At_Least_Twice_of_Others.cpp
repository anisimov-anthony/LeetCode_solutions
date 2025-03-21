#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
  public:
    int dominantIndex(std::vector<int> &nums) {
        if (nums.empty()) {
            return -1;
        }

        auto max_iter = std::max_element(nums.begin(), nums.end());
        int max_value = *max_iter;
        int max_index = std::distance(nums.begin(), max_iter);

        for (size_t i = 0; i < nums.size(); i++) {
            if (i != max_index && max_value < 2 * nums[i]) {
                return -1;
            }
        }
        return max_index;
    }
};

int main() {
    Solution sol;
    std::vector<int> nums = {3, 6, 1, 0};
    std::cout << sol.dominantIndex(nums) << std::endl;
}
