#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
public:
    int searchInsert(std::vector<int>& nums, int target) {
        auto it = std::find(nums.begin(), nums.end(), target);
        
        if (it != nums.end()) {
            return std::distance(nums.begin(), it);
        }
        
        
        auto i = 0;
        while (i < nums.size() && nums[i] < target) {
            i++;
        }
        return i;
    }
};

int main() {
    Solution sol;
    std::vector<int> nums = {1, 3, 5, 6};
    int target = 7;
    std::cout << sol.searchInsert(nums, target) << std::endl; 
    return 0;
}
