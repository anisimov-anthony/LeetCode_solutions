#include <iostream>
#include <string>
#include <stack>

class Solution {
public:
    bool isValid(std::string s) {
        std::stack<char> st;

        for (size_t i = 0; i < s.size(); i++) {
            if (s[i] == '(' || s[i] == '{' || s[i] == '[') {
                st.push(s[i]);
            } else if (s[i] == ')') {
                if (!st.empty() && st.top() == '(') {
                    st.pop();
                } else {
                    return false;
                }
            } else if (s[i] == '}') {
                if (!st.empty() && st.top() == '{') {
                    st.pop();
                } else {
                    return false;
                }
            } else if (s[i] == ']') {
                if (!st.empty() && st.top() == '[') {
                    st.pop();
                } else {
                    return false;
                }
            }
        }

        return st.empty(); 
    }
};

int main() {
    std::string str = "(]";
    Solution sol;
    std::cout << sol.isValid(str); 
}
