#include <algorithm>
#include <unordered_set>
#include <vector>

class Solution {
  public:
    int findMaxK(std::vector<int> &nums) {
        std::unordered_set<int> negs;
        std::vector<int> poss_arr;

        for (size_t i = 0; i < nums.size(); i++) {
            if (nums[i] > 0) {
                poss_arr.push_back(nums[i]);
            } else {
                negs.insert(nums[i]);
            }
        }

        int max = -1;
        for (size_t i = 0; i < poss_arr.size(); i++) {
            if (negs.find(-poss_arr[i]) != negs.end() && poss_arr[i] > max) {
                max = poss_arr[i];
            }
        }

        return max;
    }
};