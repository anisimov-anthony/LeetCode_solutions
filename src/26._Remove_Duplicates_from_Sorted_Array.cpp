#include <iostream>
#include <vector>

class Solution {
public:
    int removeDuplicates(std::vector<int>& nums) {
        auto j = 0;
        for (size_t i = 1; i < nums.size(); i++)
        {
            if (nums[j] < nums[i])
            {
                nums[j+1] = nums[i];
                
                j++;
            }
            
        }
        return j + 1;
        
    }
};

int main() {
    std::vector<int> nums = {1, 1, 2};
    Solution sol;
    std::cout << sol.removeDuplicates(nums);
}