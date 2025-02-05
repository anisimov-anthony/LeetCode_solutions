#include <iostream>
#include <string>
#include <limits>

class Solution {
public:
    int reverse(int x) {
        auto str_x = std::to_string(x);
        std::string rev_x = "";
        for (auto it = str_x.rbegin(); it != str_x.rend(); it++)
        {
            if (*it != '-')
            {
                rev_x.push_back(*it);
            }
            
            
        }
        
        auto res_x = std::stol(rev_x);
        if (x < 0)
        {
            res_x *= -1;
        }

        if (res_x < std::numeric_limits<int>::min() || res_x > std::numeric_limits<int>::max()) {
            return 0;
        }
        
        return res_x;
        
    }
};

int main() {
    auto num = -12;
    Solution sol;
    
    std::cout << sol.reverse(num);
}