#include <iostream>
#include <cmath>

class Solution {
public:
    int mySqrt(int x) {
        return std::sqrt(x);
    }
};

int main() {
    auto x = 121;
    Solution sol;
    std::cout << sol.mySqrt(x);
}