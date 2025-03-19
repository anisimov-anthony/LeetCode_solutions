#include <string>

class Solution {
  public:
    int furthestDistanceFromOrigin(std::string moves) {
        auto spaces = 0;
        auto lefts = 0;
        auto rights = 0;
        for (auto &i : moves) {
            if (i == '_') {
                ++spaces;
            } else if (i == 'L') {
                ++lefts;
            } else {
                ++rights;
            }
        }

        auto shift_LR = lefts - rights;

        return std::abs(shift_LR) + spaces;
    }
};