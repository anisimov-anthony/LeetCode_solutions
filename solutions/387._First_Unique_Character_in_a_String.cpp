#include <iostream>
#include <unordered_map>
#include <string>

class Solution {
public:
    int firstUniqChar(const std::string& s) {
        std::unordered_map<char, int> characters;
        
        for (char ch : s) {
            characters[ch]++;
        }
        
        for (size_t i = 0; i < s.size(); ++i) {
            if (characters[s[i]] == 1) {
                return i;
            }
        }
        
        return -1;
    }
};

int main() {
    std::string s = "leetcode";
    Solution sol;
    std::cout << sol.firstUniqChar(s) << std::endl;
}
