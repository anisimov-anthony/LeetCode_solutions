#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <unordered_map>

class Solution {
public:
    std::vector<std::vector<std::string>> groupAnagrams(std::vector<std::string>& strs) {
        std::unordered_map<std::string, std::vector<std::string>> anagrams;
        for (auto& str : strs) {
            std::string sorted = str;
            std::sort(sorted.begin(), sorted.end());
            anagrams[sorted].push_back(str);
        }
        
        std::vector<std::vector<std::string>> result;
        for (auto& pair : anagrams) {
            result.push_back(pair.second);
        }
        
        return result;
    }
};

int main() {
    std::vector<std::string> strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
    Solution sol;
    std::vector<std::vector<std::string>> result = sol.groupAnagrams(strs);
    
    for (const auto& group : result) {
        for (const auto& word : group) {
            std::cout << word << " ";
        }
        std::cout << std::endl;
    }

    return 0;
}
