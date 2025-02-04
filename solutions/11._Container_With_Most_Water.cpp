#include <iostream>
#include <vector>
#include <cmath>
#include <algorithm>

class Solution {
public:
    auto maxArea(std::vector<int>& height) {

        auto max_square = 0;
        auto left = 0;
        auto right = height.size() - 1;
        while (left < right) {

            auto min_height = std::min(height[left], height[right]);
            auto length = static_cast<int>(right - left);
            max_square = std::max(max_square, min_height * length);
            
            if (height[left] < height[right]) {
                ++left;
            } else {
                --right;
            }
        }
    return max_square;
        
    }
};

int main() {
    std::vector<int> height = {1,8,6,2,5,4,8,3,7};
    Solution sol;
    std::cout << sol.maxArea(height);
}