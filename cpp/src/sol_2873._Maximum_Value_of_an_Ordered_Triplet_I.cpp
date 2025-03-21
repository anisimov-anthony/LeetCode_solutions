#include <vector>

class Solution {
  public:
    long long maximumTripletValue(std::vector<int> &nums) {

        long long int result = 0;

        for (size_t i = 0; i < nums.size(); i++) {
            for (size_t j = i + 1; j < nums.size(); j++) {
                for (size_t k = j + 1; k < nums.size(); k++) {
                    long long int triplet = static_cast<long long>(nums[i] - nums[j]) * nums[k];
                    if (triplet > result) {
                        result = triplet;
                    }
                }
            }
        }

        return result;
    }
};