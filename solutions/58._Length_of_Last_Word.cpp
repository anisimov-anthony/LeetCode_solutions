#include <iostream>
#include <string>
#include <vector>
#include <sstream>

class Solution {
public:
    auto lengthOfLastWord(std::string s) {
        std::vector<std::string> strings;
        std::stringstream ss(s);
        std::string word;

        while (ss >> word)
        {
            strings.push_back(word);
        }

        return strings[strings.size() - 1].size();
    }
};

int main() {
    auto str = "   fly me   to   the moon  ";
    Solution sol;
    std::cout << sol.lengthOfLastWord(str);
}