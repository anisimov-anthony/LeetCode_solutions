#include <vector>
#include <iostream>

class Solution {
public:
    int maxProfit(std::vector<int>& prices) {
        auto lower_bound = prices[0];
        auto profit = 0;

        for (int i = 1; i < prices.size(); i++) 
        {
            if (prices[i] < lower_bound) 
            {
                lower_bound = prices[i];
            } 
            else if (prices[i] - lower_bound > profit) 
            {
                profit = prices[i] - lower_bound;
            }
        }
        return profit;
    }
};

int main() {
    std::vector<int> dt = {7,6,4,3,1};
    Solution sol;
    std::cout << sol.maxProfit(dt);
    /*
    for (auto &&i : sol.maxProfit(dt))
    {
        std::cout << i << std::endl;
    }
    */
}