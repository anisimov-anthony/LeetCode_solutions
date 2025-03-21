#include <iostream>
#include <vector>

class Solution {
public:
    int majorityElement(std::vector<int>& nums) {
        auto result = 0;
        auto counter = 0;

        for (int i : nums) {
            if (counter == 0) {
                result = i;
            }
            counter += (i == result) ? 1 : -1;
        }

        return result;
    }
};


