#include <iostream>
#include <string>

class Solution {
  public:
    int strStr(std::string haystack, std::string needle) {
        if (haystack.size() < needle.size()) {
            return -1;
        }

        if (needle.empty()) {
            return 0;
        }

        for (size_t i = 0; i <= haystack.size() - needle.size(); i++) {
            if (haystack.substr(i, needle.size()) == needle) {
                return i;
            }
        }
        return -1;
    }
};

int main() {
    std::string haystack = "leetcode";
    std::string needle = "leeto";
    Solution sol;
    std::cout << sol.strStr(haystack, needle) << std::endl;
}
