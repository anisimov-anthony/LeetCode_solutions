#include <iostream>
class Solution {
public:
    bool isPowerOfTwo(int n) {
        int counter = 0;
        while (n > 0) {
            counter += n & 1; 
            n >>= 1; 
        }
        if (counter == 1)
        {
            return true;
        }
        else
        {
            return false;
        }
        
        
        
    }
};

int main() {
    auto n = 1023;
    Solution sol;
    std::cout << sol.isPowerOfTwo(n);
}