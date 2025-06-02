#include <iostream>

int main() {
    int* arr = new int[10]; // Manual heap allocation
    for (int i = 0; i < 10; ++i) {
        arr[i] = i * 10;
    }

    std::cout << "Element at index 5: " << arr[5] << std::endl;

    delete[] arr; // Mmemory deallocation
    arr = nullptr;

    return 0;
}
