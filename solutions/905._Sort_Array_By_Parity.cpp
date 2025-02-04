#include <algorithm>
#include <vector>

class Solution {
  public:
    std::vector<int> sortArrayByParity(std::vector<int> &nums) {
        std::partition(nums.begin(), nums.end(), [](int num) {
            return num % 2 == 0;
        });
        return nums;
    }
};