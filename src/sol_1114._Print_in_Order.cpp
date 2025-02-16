#include <condition_variable>
#include <functional>
#include <mutex>

class Foo {
  private:
    std::mutex m;
    std::condition_variable cv;
    std::uint8_t output_stage;

  public:
    Foo() {
        output_stage = 0;
    }

    void first(std::function<void()> printFirst) {

        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        output_stage = 1;
        cv.notify_all();
    }

    void second(std::function<void()> printSecond) {

        std::unique_lock<std::mutex> lk(m);
        cv.wait(lk, [this] { return output_stage == 1; });
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        output_stage = 2;
        cv.notify_all();
    }

    void third(std::function<void()> printThird) {

        std::unique_lock<std::mutex> lk(m);
        cv.wait(lk, [this] { return output_stage == 2; });
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
    }
};