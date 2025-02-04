#include <vector>
#include <string>
#include <iostream>

class Solution {
public:
    int minOperations(std::vector<std::string>& logs) {
        auto counter = 0;
        for (size_t i = 0; i < logs.size(); i++)
        {
            if (logs[i] == "../")
            {
                counter--;
                if (counter < 0)
                {
                    counter = 0;
                }
            }
            else if (logs[i] != "./")
            {
                counter++;
            }
                        
        }
        if (counter < 0)
        {
            return 0;
        }
        
        return counter++;
        
    }
};

int main() {
    std::vector<std::string> logs = {"d1/","d2/","./","d3/","../","d31/"};
    Solution sol;
    std::cout << sol.minOperations(logs);

}