#include <string>

class Solution {
  public:
    bool hasSpecialSubstring(std::string s, int k) {
        if (s.size() < k) {
            return false;
        }

        for (size_t i = 0; i <= s.size() - k; ++i) {
            bool all_same = true;
            char c = s[i];
            for (size_t j = i + 1; j < i + k; ++j) {
                if (s[j] != c) {
                    all_same = false;
                    break;
                }
            }

            if (all_same) {
                bool valid_before = (i == 0) || (s[i - 1] != c);
                bool valid_after = (i + k == s.size()) || (s[i + k] != c);

                if (valid_before && valid_after) {
                    return true;
                }
            }
        }

        return false;
    }
};