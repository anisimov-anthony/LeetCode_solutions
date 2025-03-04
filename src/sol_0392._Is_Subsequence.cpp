#include <string>

class Solution {
  public:
    bool isSubsequence(std::string s, std::string t) {
        size_t j = 0;
        for (size_t i = 0; i < s.size(); i++) {
            while (j < t.size() && t[j] != s[i]) {
                j++;
            }
            if (j == t.size()) {
                return false;
            }
            j++;
        }
        return true;
    }
};
