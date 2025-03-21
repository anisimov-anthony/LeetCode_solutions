#include <string>
class Solution {
  private:
    bool is_palindrome(const std::string &s) {
        auto left = 0;
        auto right = s.size() - 1;
        while (left < right) {
            if (s[left] != s[right]) {
                return false;
            }
            left++;
            right--;
        }
        return true;
    }

  public:
    int removePalindromeSub(std::string s) {
        // corner cases
        if (s.empty()) {
            return 0;
        }
        if (is_palindrome(s)) {
            return 1;
        }

        // main case
        return 2;
    }
};