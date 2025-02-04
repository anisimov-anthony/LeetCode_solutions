#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <cmath>

class Solution {
public:
    const std::map<int, std::string> simple_numerals{   {1,    "I" },
                                                        {5,    "V" },
                                                        {10,   "X" },
                                                        {50,   "L" }, 
                                                        {100,  "C" },  
                                                        {500,  "D" },
                                                        {1000, "M" } };

    const std::map<int, std::string> comp_numerals{ {4,    "IV"},
                                                    {6,    "VI"},
                                                    {7,    "VII"},
                                                    {8,    "VIII"},
                                                    {9,    "IX"},
                                                    {40,   "XL"},  
                                                    {60,   "LX"}, 
                                                    {70,   "LXX"}, 
                                                    {80,   "LXXX"}, 
                                                    {90,   "XC"},  
                                                    {400,  "CD"},
                                                    {600,  "DC"},
                                                    {700,  "DCC"},
                                                    {800,  "DCCC"},
                                                    {900,  "CM"}, };    

    std::string intToRoman(int num) {
        auto num_str = std::to_string(num);
        std::string result = "";
        if (comp_numerals.find(num) != comp_numerals.end())
        {
            return comp_numerals.find(num)->second;
        }
        if (simple_numerals.find(num) != simple_numerals.end())
        {
            return simple_numerals.find(num)->second;
        }

        auto digit_ind = 0;
        for (auto it = num_str.begin(); it != num_str.end(); it++)
        {
            auto summand = std::stoi(std::string(1, *it)) * std::pow(10, num_str.size() - digit_ind - 1);
            
            auto find_it = comp_numerals.find(summand);
            if (find_it != comp_numerals.end())
            {
                result += find_it->second;
                digit_ind++;
            }
            else
            {
                if (simple_numerals.find(summand) != simple_numerals.end())
                {
                    result += simple_numerals.find(summand)->second;
                    digit_ind++;
                }
                else
                {
                    if (summand == 0)
                    {
                        digit_ind++;
                        continue;
                    }
                    
                    auto multiplier = std::pow(10, num_str.size() - digit_ind - 1);
                    while (multiplier <= summand + std::pow(10, num_str.size() - digit_ind - 1))
                    {
                        result += simple_numerals.find(std::pow(10, num_str.size() - digit_ind - 1))->second;
                        multiplier += multiplier;
                    }
                    
                    /* code */
                    digit_ind++;
                }
                
            }
            
            
            
        }
        return result;
        
        

  }
};

int main() {
    auto number = 1994;
    Solution sol;
    std::cout << sol.intToRoman(number);
}