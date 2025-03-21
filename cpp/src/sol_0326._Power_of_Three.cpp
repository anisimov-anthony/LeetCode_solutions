class Solution {
  public:
    bool isPowerOfThree(int n) {
        if (n <= 0) {
            return false;
        }
        if (n == 1) {
            return true;
        }

        long int tmp = 1;
        while (tmp <= n) {
            tmp = tmp * 3;

            if (tmp == n) {
                return true;
            }
        }
        return false;
    }
};