#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
public:
    std::vector<int> intersect(std::vector<int>& nums1, std::vector<int>& nums2) {

        std::sort(nums1.begin(), nums1.end(), std::greater());
        std::sort(nums2.begin(), nums2.end(), std::greater());

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
                nums2[i] = -1;
                *it = -1;
            }
            
            
        }
        return result;

        
    }
};