#include <iostream>
#include <vector>

class Solution {
public:
    std::vector<int> getRow(int rowIndex) {
        std::vector<std::vector<int>> result = {{1}};
        
        // Если rowIndex равен 0, сразу возвращаем первый элемент
        if (rowIndex == 0) {
            return result[0];
        }

        for (int i = 1; i <= rowIndex; ++i) {
            std::vector<int> row_data;
            row_data.push_back(1); // Первый элемент всегда 1

            for (int j = 1; j < i; ++j) {
                row_data.push_back(result[i - 1][j - 1] + result[i - 1][j]);
            }

            row_data.push_back(1); // Последний элемент всегда 1
            result.push_back(row_data);
        }

        return result[rowIndex];
    }
};

