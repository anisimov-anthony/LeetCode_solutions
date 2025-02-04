#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
  public:
    std::vector<std::vector<int>> minimumAbsDifference(std::vector<int> &arr) {
        std::sort(arr.begin(), arr.end());
        int min_diff = arr[1] - arr[0];
        for (size_t i = 1; i < arr.size() - 1; ++i) {
            int diff = arr[i + 1] - arr[i];
            if (diff < min_diff) {
                min_diff = diff;
            }
        }
        std::vector<std::vector<int>> result;
        for (size_t i = 0; i < arr.size() - 1; ++i) {
            if (arr[i + 1] - arr[i] == min_diff) {
                result.push_back({arr[i], arr[i + 1]});
            }
        }
        return result;
    }
};