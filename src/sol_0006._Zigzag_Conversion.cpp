#include <string>
#include <vector>
class Solution {
  public:
    std::string convert(std::string s, int numRows) {
        if (numRows == 1 || numRows >= s.length()) {
            return s;
        }

        auto idx = 0;
        auto direction = 1;
        std::vector<std::vector<char>> rows(numRows);

        for (auto symb : s) {
            rows[idx].push_back(symb);
            if (idx == 0) {
                direction = 1;
            } else if (idx == numRows - 1) {
                direction = -1;
            }
            idx += direction;
        }

        std::string result;
        for (const auto &row : rows) {
            for (auto symb : row) {
                result += symb;
            }
        }

        return result;
    }
};