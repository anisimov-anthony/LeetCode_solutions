#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <unordered_map>
#include <unordered_set>

class Solution {
public:
    bool wordPattern(std::string pattern, std::string s) {
        std::vector<std::string> words;
        std::istringstream iss(s);
        std::string word;
        
        while (iss >> word) {
            words.push_back(word);
        }

        if (pattern.size() != words.size()) {
            return false;
        }

        std::unordered_map<char, std::string> ch_wr;
        std::unordered_map<std::string, char> wr_ch;

        for (size_t i = 0; i < pattern.size(); ++i) {
            char p = pattern[i];
            const std::string& w = words[i];

            if (ch_wr.find(p) != ch_wr.end() && ch_wr[p] != w) {
                return false;
            }
            if (wr_ch.find(w) != wr_ch.end() && wr_ch[w] != p) {
                return false;
            }

            ch_wr[p] = w;
            wr_ch[w] = p;
        }

        return true;
    }
};

int main() {
    std::string pattern = "abba";
    std::string s = "dog cat cat dog";
    Solution sol;
    std::cout << std::boolalpha << sol.wordPattern(pattern, s) << std::endl;
}
