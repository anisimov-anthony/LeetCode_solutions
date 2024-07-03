#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <unordered_set>

class Solution {
public:
    int lengthOfLongestSubstring(std::string s) {
        if (s.empty()) {
            return 0;
        }
        
        std::vector<int> sizes;
        for (size_t i = 0; i < s.size(); i++)
        {
            std::unordered_set<char> substr_symbols;
            
            auto j = i;
            while (!substr_symbols.contains(s[j]) && j != s.size())
            {
                substr_symbols.insert(s[j]);
                ++j;

            }
            sizes.push_back(substr_symbols.size());
        }
        return *std::max_element(sizes.begin(), sizes.end(), [](int a, int b)
        {
            return a < b;
        });
        
    }
};

int main() {
    auto s = "abcabcbb";
    Solution sol;
    std::cout << sol.lengthOfLongestSubstring(s);
}