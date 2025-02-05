#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
public:
    std::vector<int> intersection(std::vector<int>& nums1, std::vector<int>& nums2) {

        std::sort(nums1.begin(), nums1.end(), std::greater());
        std::sort(nums2.begin(), nums2.end(), std::greater());

        auto last1 = std::unique(nums1.begin(), nums1.end());
        auto last2 = std::unique(nums2.begin(), nums2.end());

        nums1.erase(last1, nums1.end());
        nums2.erase(last2, nums2.end());

        if (nums1.size() < nums2.size())
        {
            std::swap(nums1, nums2);
        }
        
        std::vector<int> result;

        for (size_t i = 0; i < nums2.size(); i++)
        {
            auto it = std::find(nums1.begin(), nums1.end(), nums2[i]);
            if (it != nums1.end())
            {
                result.push_back(nums2[i]);
            }
            
            
        }
        return result;
        
        
        


        
    }
};