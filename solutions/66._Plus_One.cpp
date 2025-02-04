#include <iostream>
#include <vector>

class Solution {
public:
    std::vector<int> plusOne(std::vector<int>& v) {
        for(int i = v.size()-1; i >= 0; i--){
            if(i == v.size()-1)
                v[i]++;
            if(v[i] == 10){
                v[i] = 0;
                if(i != 0){
                    v[i-1]++;
                }
                else{
                    v.push_back(0);
                    v[i] = 1;
                }
            }
        }
        return v;
    }
};

int main() {
    std::vector<int> digits = {1, 2, 3};
    Solution sol;
    std::vector<int> result = sol.plusOne(digits);
    for (int digit : result) {
        std::cout << digit << " ";
    }
    std::cout << std::endl;

    return 0;
}
