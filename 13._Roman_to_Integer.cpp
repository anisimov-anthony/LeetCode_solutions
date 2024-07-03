#include <vector>
#include <iostream>
#include <map>

class Solution
{
public:
    int romanToInt(const std::string &str)
    {

        const std::map<char, int> numerals{{'I', 1},
                                           {'V', 5},
                                           {'X', 10},
                                           {'L', 50},
                                           {'C', 100},
                                           {'D', 500},
                                           {'M', 1000}};
        std::vector<int> numbers;

        for (char c : str)
        {
            numbers.push_back(numerals.at(c));
        }

        int result = 0;
        for (size_t i = 0; i < numbers.size(); ++i)
        {
            if (i + 1 < numbers.size() && numbers[i] < numbers[i + 1])
            {
                result -= numbers[i];
            }
            else
            {
                result += numbers[i];
            }
        }

        return result;
    }
};
