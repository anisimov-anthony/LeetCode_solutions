#include <cctype>
#include <string>

class Solution {
  public:
    std::string makeGood(std::string s) {
        // corner cases
        if (s.empty() || s.size() == 1) {
            return s;
        }

        // main case
        std::string result;
        result.push_back(s[0]);

        for (size_t i = 1; i < s.size(); i++) {
            if (!result.empty() && tolower(s[i]) == tolower(result.back()) && s[i] != result.back()) {
                result.pop_back();
            } else {
                result.push_back(s[i]);
            }
        }

        return result;
    }
};
