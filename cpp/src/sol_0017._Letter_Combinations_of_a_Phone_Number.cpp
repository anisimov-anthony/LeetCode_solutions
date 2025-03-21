#include <string>
#include <unordered_map>
#include <vector>

class Solution {
  public:
    std::vector<std::string> letterCombinations(std::string digits) {

        std::unordered_map<char, std::string> domofon = {
            {'2', "abc"},
            {'3', "def"},
            {'4', "ghi"},
            {'5', "jkl"},
            {'6', "mno"},
            {'7', "pqrs"},
            {'8', "tuv"},
            {'9', "wxyz"}};

        std::vector<std::string> result;

        // corner case
        if (digits.empty()) {
            return result;
        }

        // main case
        std::string current;
        combine(0, digits, domofon, current, result);
        return result;
    }

  private:
    void combine(
        int index,
        const std::string &digits,
        const std::unordered_map<char, std::string> &map,
        std::string &current,
        std::vector<std::string> &result) {

        if (index == digits.size()) {
            result.push_back(current);
            return;
        }

        for (char c : map.at(digits[index])) {
            current.push_back(c);
            combine(index + 1, digits, map, current, result);
            current.pop_back();
        }
    }
};
