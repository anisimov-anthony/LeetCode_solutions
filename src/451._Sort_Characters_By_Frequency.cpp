#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
#include <algorithm>

struct freq {
    char character;
    int frequency;
};

bool compare(const freq &a, const freq &b) {
    return a.frequency > b.frequency;
}

class Solution {
public:
    std::string frequencySort(std::string s) {
        std::unordered_map<char, int> freq_dt;

        for (char c : s) {
            freq_dt[c]++;
        }

        std::vector<freq> freq_cmp;
        for (const auto &pair : freq_dt)
         {
            freq_cmp.push_back({pair.first, pair.second});
        }

        std::sort(freq_cmp.begin(), freq_cmp.end(), compare);

        std::string result;
        for (const auto &entry : freq_cmp) {
            result.append(entry.frequency, entry.character);
        }

        return result;
    }
};

