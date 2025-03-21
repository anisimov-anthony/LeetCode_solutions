#include <unordered_set>

class Solution {
  private:
    int sum_of_squares(int n) {
        int sum = 0;
        while (n > 0) {
            int digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        return sum;
    }

  public:
    bool isHappy(int n) {
        std::unordered_set<int> seen;

        while (n != 1) {
            n = sum_of_squares(n);

            if (seen.find(n) != seen.end()) {
                return false;
            }

            seen.insert(n);
        }

        return true;
    }
};
