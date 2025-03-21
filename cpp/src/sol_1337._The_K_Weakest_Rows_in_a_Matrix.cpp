#include <algorithm>
#include <unordered_map>
#include <vector>

class Solution {
  public:
    std::vector<int> kWeakestRows(std::vector<std::vector<int>> &mat, int k) {
        std::vector<std::pair<int, int>> st;
        for (size_t i = 0; i < mat.size(); i++) {
            auto num_soliders = 0;
            for (size_t j = 0; j < mat[0].size(); j++) {
                if (mat[i][j] == 1) {
                    num_soliders += 1;
                }
            }
            st.emplace_back(num_soliders, i);
        }
        std::sort(st.begin(), st.end());
        std::vector<int> result;
        for (int i = 0; i < k; i++) {
            result.push_back(st[i].second);
        }

        return result;
    }
};