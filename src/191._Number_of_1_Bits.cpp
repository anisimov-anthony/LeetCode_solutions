#include <iostream>

class Solution {
public:
    int hammingWeight(int n) {
    int counter = 0;
    while (n > 0) {
        counter += n & 1; 
        n >>= 1; 
    }
    return counter;
        
    }
};

int main() {
    auto number = 11;
    Solution sol;
    std::cout << sol.hammingWeight(number);
}