#include <algorithm>
#include <vector>

class Solution {
  public:
    std::vector<int> advantageCount(std::vector<int> &nums1, std::vector<int> &nums2) {
        std::vector<int> result;
        std::sort(nums1.begin(), nums1.end());
        for (int i = 0; i < nums2.size(); i++) {

            auto it = nums1.back() > nums2[i] ? upper_bound(nums1.begin(), nums1.end(), nums2[i]) : nums1.begin();
            result.push_back(*it);
            nums1.erase(it);
        }
        return result;
    }
};