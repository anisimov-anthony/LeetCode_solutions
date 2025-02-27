#include <algorithm>
#include <queue>
#include <vector>

class Solution {
  private:
    bool similar_elements(const std::queue<int> &input) {
        auto working_queue = input;

        if (input.empty()) {
            return false;
        }

        auto first_elem = working_queue.front();

        for (size_t i = 0; i < working_queue.size(); i++) {
            if (working_queue.front() != first_elem) {
                return false;
            }

            working_queue.push(working_queue.front());
            working_queue.pop();
        }

        return true;
    }

  public:
    int countStudents(std::vector<int> &students, std::vector<int> &sandwiches) {
        // corner case
        if (students.empty() || sandwiches.empty()) {
            return 0;
        }

        // main case
        std::queue<int> students_q;
        std::queue<int> sandwiches_q;

        // students to queue
        for (size_t i = 0; i < students.size(); i++) {
            students_q.push(students[i]);
        }
        // sandwiches to queue
        for (size_t i = 0; i < sandwiches.size(); i++) {
            sandwiches_q.push(sandwiches[i]);
        }

        while (!sandwiches_q.empty()) {
            if (students_q.front() == sandwiches_q.front()) {
                students_q.pop();
                sandwiches_q.pop();
            } else {
                if (similar_elements(students_q) && sandwiches_q.front() != students_q.front()) {
                    break;
                }

                students_q.push(students_q.front());
                students_q.pop();
            }
        }

        return students_q.size();
    }
};
