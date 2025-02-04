#include <iostream>
#include <vector>

class Solution {
  public:
    void moveZeroes(std::vector<int> &nums) {
        auto no_zero = 0;

        for (int i = 0; i < nums.size(); ++i) {
            if (nums[i] != 0) {
                std::swap(nums[no_zero++], nums[i]);
            }
        }
    }
};