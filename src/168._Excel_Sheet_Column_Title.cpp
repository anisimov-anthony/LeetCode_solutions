#include <string>
#include <algorithm>
class Solution {
public:
    std::string convertToTitle(int columnNumber) {
        std::string result = "";
        while (columnNumber > 0)
        {
            columnNumber--;
            auto digit = columnNumber%26;
            columnNumber /= 26;
            result.push_back(digit + 'A');

        }
        std::reverse(result.begin(), result.end());
        return result;
        
    }
};