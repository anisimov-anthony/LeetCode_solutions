#include <string>

class Solution {
  public:
    bool isPalindrome(std::string s) {
        std::string result;
        for (size_t i = 0; i < s.size(); i++) {
            if (std::isalpha(s[i]) || std::isdigit(s[i])) {
                result.push_back(std::tolower(s[i]));
            }
        }
        for (size_t i = 0; i < result.size() / 2; i++) {
            if (result[i] != result[result.size() - i - 1]) {
                return false;
            }
        }
        return true;
    }
};