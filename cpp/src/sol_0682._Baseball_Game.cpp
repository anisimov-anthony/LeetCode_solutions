#include <string>
#include <vector>

class Solution {
  public:
    int calPoints(std::vector<std::string> &operations) {
        std::vector<int> scores;

        for (const auto &operation : operations) {
            if (operation == "+") {
                if (scores.size() < 2) continue;
                scores.push_back(scores[scores.size() - 1] + scores[scores.size() - 2]);
                continue;
            }

            if (operation == "C") {
                if (!scores.empty()) scores.pop_back();
                continue;
            }

            if (operation == "D") {
                if (!scores.empty()) scores.push_back(2 * scores.back());
                continue;
            }

            scores.push_back(std::stoi(operation));
        }

        auto sum = 0;
        for (size_t i = 0; i < scores.size(); i++) {
            sum += scores[i];
        }
        return sum;
    }
};