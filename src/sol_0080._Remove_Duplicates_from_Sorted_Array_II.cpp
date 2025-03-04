#include <algorithm>
#include <unordered_map>
#include <vector>

class Solution {
  private:
    void mark_duplicates(std::vector<int> &nums) {
        auto marker = nums[0] - 1;
        std::unordered_map<int, int> count_map;

        for (int &num : nums) {
            count_map[num]++;
            if (count_map[num] > 2) {
                num = marker;
            }
        }
    }

    void remove_prefix(std::vector<int> &nums, int x) {
        auto it = std::find_if(nums.begin(), nums.end(), [x](int num) { return num != x; });
        nums.erase(nums.begin(), it);
    }

  public:
    int removeDuplicates(std::vector<int> &nums) {
        auto marker = nums[0] - 1;
        mark_duplicates(nums);
        std::sort(nums.begin(), nums.end(), std::less());
        remove_prefix(nums, marker);
        return nums.size();
    }
};