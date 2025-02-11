#include <cstdint>
class Solution {
  public:
    uint32_t reverseBits(uint32_t n) {
        uint32_t result = 0;
        auto pos = 32;
        while (pos--) {
            auto bit = n & 1;
            result = (result << 1) | bit;
            n = n >> 1;
        }
        return result;
    }
};