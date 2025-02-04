#include <iostream>
#include <string>
#include <algorithm>

class Solution {
public:
    bool isAnagram(std::string s, std::string t) {
        std::sort(s.begin(), s.end(), std::greater());
        std::sort(t.begin(), t.end(), std::greater());
        if (s == t)
        {
            return true;
        }
        else 
        {
            return false;
        }
        
        
    }
};

int main() {
    auto s = "rat";
    auto t = "cat";
    Solution sol;
    std::cout << sol.isAnagram(s, t);
}