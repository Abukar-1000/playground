#include <iostream>

int fibRec(int n){

    if (n <= 2){
        return 1;
    } else {
        return fibRec(n - 1) + fibRec(n -2);
    }
}

int main() {
    // solve fib itteratively

    int num1 = 0;
    int num2 = 1;
    int container = 0;
    int nThIndex = 10;
    for (int i = 1; i < nThIndex; ++i){
        container = num2;
        num2 += num1;
        num1 = container;
        std::cout << i <<") loop num1 is " << container << " num2 is " << num2 << "\n";
    }
    std::cout << nThIndex << "th Fib number is " << num2 << "\n";
    std::cout << nThIndex << "th recursive Fib number is " << fibRec(nThIndex);
    return 0;
}