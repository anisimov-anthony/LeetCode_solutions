#include <stack>
class MyQueue {
  public:
    MyQueue() {
    }

    void push(int x) {
        st1.push(x);
    }

    int pop() {
        if (st2.size() <= 1) {
            reallocate_stacks();
        }

        // case for non-empty front stack
        auto pop_elem = this->peek();
        st2.pop();
        return pop_elem;
    }

    int peek() {
        if (st2.size() <= 1) {
            reallocate_stacks();
        }
        return st2.top();
    }

    bool empty() {
        return st1.empty() && st2.empty();
    }

  private:
    std::stack<int> st1; // back of queue
    std::stack<int> st2; // front of queue

    void reallocate_stacks() {
        // realocate all elements from back stack to front stack
        std::stack<int> tmp_1;
        std::stack<int> tmp_2;

        while (!st1.empty()) {
            tmp_1.push(st1.top());
            st1.pop();
        }
        while (!st2.empty()) {
            tmp_2.push(st2.top());
            st2.pop();
        }

        while (!tmp_1.empty()) {
            tmp_2.push(tmp_1.top());
            tmp_1.pop();
        }

        while (!tmp_2.empty()) {
            st2.push(tmp_2.top());
            tmp_2.pop();
        }
    }
};

/**
 * Your MyQueue object will be instantiated and called as such:
 * MyQueue* obj = new MyQueue();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->peek();
 * bool param_4 = obj->empty();
 */