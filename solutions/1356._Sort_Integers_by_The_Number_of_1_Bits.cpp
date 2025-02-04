#include <algorithm>
#include <vector>

class Solution {
  public:
    auto count_bits(int number) {
        auto count = 0;
        while (number) {
            count += number & 1;
            number >>= 1;
        }
        return count;
    }
    std::vector<int> sortByBits(std::vector<int> &arr) {
        auto comp = [&](int a, int b) { return count_bits(a) < count_bits(b); };
        std::sort(arr.begin(), arr.end(), comp);
        return arr;
    }
};