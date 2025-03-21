#include <iostream>
#include <string>

class Solution {
public:
    std::string longestPalindrome(std::string s) {
        if (s.empty()) return "";
        int n = s.size();
        int start = 0, maxLength = 1;

        auto expandAroundCenter = [&](int left, int right) {
            while (left >= 0 && right < n && s[left] == s[right]) {
                --left;
                ++right;
            }
            int length = right - left - 1;
            if (length > maxLength) {
                start = left + 1;
                maxLength = length;
            }
        };

        for (int i = 0; i < n; ++i) {
            expandAroundCenter(i, i);       // Odd length palindromes
            expandAroundCenter(i, i + 1);   // Even length palindromes
        }

        return s.substr(start, maxLength);
    }
};

int main() {
    std::string s = "cbbd";
    Solution sol;
    std::cout << sol.longestPalindrome(s) << std::endl;
    return 0;
}
