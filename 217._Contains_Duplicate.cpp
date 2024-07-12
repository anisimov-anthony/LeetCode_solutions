#include <iostream>
#include <vector>
#include <set>

class Solution {
public:
    bool containsDuplicate(std::vector<int>& nums) {
        std::set<int> dupl;
        for (auto &&i : nums)
        {
            auto it = dupl.find(i);
            if (it != dupl.end())
            {
                return true;
            }
            else
            {
                dupl.insert(i);
            }
            
            
        }
        return false;    
    }
};

int main() {
    std::vector<int> nums = {1,2,3,1};
    Solution sol;
    std::cout << sol.containsDuplicate(nums);
}