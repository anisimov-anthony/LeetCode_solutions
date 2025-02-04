#include <vector>
#include <iostream>

class Solution {
public:
    int climbStairs(int n) {
        if (n <= 1) {
            return 1;
        }

        std::vector<int> stps(n + 1);
        stps[0] = 1;
        stps[1] = 1;

        for (int i = 2; i <= n; ++i) {
            stps[i] = stps[i - 1] + stps[i - 2];
        }

        return stps[n];
    }
};

int main() {
    Solution solution;
    int n = 3;
    std::cout << solution.climbStairs(n);
}
