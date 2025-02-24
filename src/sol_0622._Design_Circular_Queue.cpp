#include <vector>

class MyCircularQueue {
  private:
    std::vector<int> store;
    size_t rear;
    size_t front;
    size_t size;

  public:
    MyCircularQueue(int k) {
        store.resize(k + 1);
        front = 0;
        rear = 0;
        size = k + 1;
    }

    bool enQueue(int value) {
        if (isFull()) return false;

        store[front] = value;
        front = (front + 1) % size;
        return true;
    }

    bool deQueue() {
        if (isEmpty()) return false;

        rear = (rear + 1) % size;
        return true;
    }

    int Front() {
        if (isEmpty()) return -1;
        return store[rear];
    }

    int Rear() {
        if (isEmpty()) return -1;
        return store[(front + size - 1) % size];
    }

    bool isEmpty() {
        return front == rear;
    }

    bool isFull() {
        return (front + 1) % size == rear;
    }
};
