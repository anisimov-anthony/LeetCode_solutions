#include <iostream>
#include <vector>

class Solution {
  public:
    std::vector<std::vector<int>> generate(int numRows) {
        std::vector<std::vector<int>> result = {{1}};
        if (numRows == 1) {
            return result;
        }
        if (numRows == 2) {
            result.push_back({1, 1});
            return result;
        }

        for (size_t i = 2; i < numRows + 1; i++) {
            std::vector<int> row_data;
            row_data.push_back(result.back().front());
            auto first = 0;
            auto second = first + 1;
            while (second != result.back().size()) {
                row_data.push_back(result.back()[first] + result.back()[second]);
                first++;
                second++;
            }
            row_data.push_back(result.back().back());
            result.push_back(row_data);
        }
        return result;
    }
};