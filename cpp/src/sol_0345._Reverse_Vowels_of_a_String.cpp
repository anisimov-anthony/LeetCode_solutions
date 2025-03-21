#include <string>
#include <unordered_set>

class Solution {
  public:
    std::string reverseVowels(std::string s) {
        std::unordered_set<char> vowels = {'a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'};
        int left_idx = 0;
        int right_idx = s.size() - 1;

        while (left_idx < right_idx) {
            while (left_idx < right_idx && vowels.find(s[left_idx]) == vowels.end()) {
                ++left_idx;
            }
            while (left_idx < right_idx && vowels.find(s[right_idx]) == vowels.end()) {
                --right_idx;
            }

            if (left_idx < right_idx) {
                std::swap(s[left_idx], s[right_idx]);
                ++left_idx;
                --right_idx;
            }
        }

        return s;
    }
};
