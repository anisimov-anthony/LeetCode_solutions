#include <vector>
#include <iostream>

class Solution {
 public:
  void rotate(std::vector<std::vector<int>>& matrix) {
    auto n = matrix.size();
    std::vector<std::vector<int>> result(n, std::vector<int>(n));

    for (size_t i = 0; i < n; i++) {
      for (size_t j = 0; j < n; j++) {
        result[j][n - i - 1] = matrix[i][j];
      }
    }
    matrix = result;
  }
};

int main() {
  Solution sol;
  std::vector<std::vector<int>> matrix = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};

  sol.rotate(matrix);

  // Output the rotated matrix
  for (const auto& row : matrix) {
    for (const auto& elem : row) {
      std::cout << elem << " ";
    }
    std::cout << "\n";
  }

  return 0;
}