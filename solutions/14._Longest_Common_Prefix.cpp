#include <string>
#include <vector>
#include <algorithm>
#include <iostream>

class Solution {
public:
    std::string longestCommonPrefix(std::vector<std::string>& strs) {
        if (strs.empty()) return "";

        auto comparator = [](const std::string& a, const std::string& b) {
            return a.size() < b.size();
        };

        auto min_str = *std::min_element(strs.begin(), strs.end(), comparator);
        
        std::vector<std::string> prefixes;
        std::vector<std::string> substrings;

        for (size_t i = 0; i < min_str.size(); ++i) {
            for (size_t j = 1; j <= min_str.size() - i; ++j) {
                substrings.push_back(min_str.substr(i, j));
            }
        }
        
        for (size_t i = 0; i < substrings.size(); ++i) {
            auto prefix = substrings[i];
            if (std::all_of(strs.begin(), strs.end(), [&prefix](const std::string& str) {
                return str.find(prefix) == 0; 
            })) {
                prefixes.push_back(prefix);
            }
        }
        
        if (prefixes.empty()) return "";

        return *std::max_element(prefixes.begin(), prefixes.end(), comparator);
    }
};
