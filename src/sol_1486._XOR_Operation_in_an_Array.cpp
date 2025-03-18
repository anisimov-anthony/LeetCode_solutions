#include <vector>

class Solution {
  public:
    int xorOperation(int n, int start) {
        std::vector<int> nums(n);
        for (size_t i = 0; i < nums.size(); i++) {
            nums[i] = start + 2 * i;
        }
        auto result = nums[0];
        for (size_t i = 1; i < n; i++) {
            result = result ^ nums[i];
        }
        return result;
    }
};