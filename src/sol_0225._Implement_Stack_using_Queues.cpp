#include <queue>
class MyStack {
  public:
    MyStack() {
    }

    void push(int x) {
        q.push(x);
    }

    int pop() {
        std::queue<int> tmp;
        int pop_elem = 0;

        while (q.size() > 1) {
            tmp.push(q.front());
            q.pop();
        }

        pop_elem = q.front();
        q.pop();
        q = tmp;
        return pop_elem;
    }

    int top() {
        return q.back();
    }

    bool empty() {
        return q.empty();
    }

  private:
    std::queue<int> q;
};

/**
 * Your MyStack object will be instantiated and called as such:
 * MyStack* obj = new MyStack();
 * obj->push(x);
 * int param_2 = obj->pop();
 * int param_3 = obj->top();
 * bool param_4 = obj->empty();
 */