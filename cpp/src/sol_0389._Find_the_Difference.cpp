#include <algorithm>
#include <string>

class Solution {
  public:
    char findTheDifference(std::string s, std::string t) {
        std::sort(s.begin(), s.end(), std::less());
        std::sort(t.begin(), t.end(), std::less());
        for (size_t i = 0; i < s.size(); i++) {

            if (s[i] != t[i]) {
                return t[i];
            }
        }
        return t[t.size() - 1];
    }
};