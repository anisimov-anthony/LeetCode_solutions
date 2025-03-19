#include <algorithm>
#include <string>

class Solution {
  private:
    char get_char(std::string character) {
        int num = std::stoi(character);
        return 'a' + num - 1;
    }

  public:
    std::string freqAlphabets(std::string s) {
        std::string result;
        while (s.size() > 0) {
            if (s.back() == '#') {
                s.pop_back();
                std::string character;
                character.push_back(s[s.size() - 2]);
                character.push_back(s[s.size() - 1]);
                s.pop_back();
                s.pop_back();
                result.push_back(get_char(character));
            } else {
                std::string character(1, s.back());
                s.pop_back();
                result.push_back(get_char(character));
            }
        }

        std::reverse(result.begin(), result.end());
        return result;
    }
};