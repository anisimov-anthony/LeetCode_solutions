#include <iostream>
#include <vector>

class Solution {
  public:
    int missingNumber(std::vector<int> &nums) {
        auto n = nums.size();
        auto true_sum = n * (n + 1) / 2;
        auto current_sum = 0;
        for (int num : nums) {
            current_sum += num;
        }
        return true_sum - current_sum;
    }
};

int main() {
    Solution sol;
    std::vector<int> nums1 = {3, 0, 1};
    std::cout << sol.missingNumber(nums1) << std::endl;
}
