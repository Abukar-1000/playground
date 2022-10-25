#include <iostream>
#include <stack>

// 10 - 99 -> 9 slots
#define SIZE 9

/*
Generate 1,000 numbers in the range [10…99] using a random number generator. Build a hash table with chaining, that can support all necessary dynamic operations, grouping the randomly generated numbers based on their first digit.
Then, for each hash table entity (slot), keep only the unique numbers, i.e., eliminate any repetitions of the same numbers. 

Your program should:

(a) include the random number generator;

(b) include an implementation of a hash table with whatever dynamic operations are needed; and

(c) should ask the user to pick a number corresponding to a table slot, and then display the corresponding entries after eliminating the repetitions.


*/

class Hash {
    private:
        // stacks will represent the linked lists for chaining since we care about looking at the most recent item
        std::stack<int> table[SIZE];

        int hashFunc(int num){
            /*
            given a number hashes it based on its magnitude calculated after dividing by 10 | also forces the result to an int
            */
           int result = num / 10;
           // account for index
           return (result - 1);
        }
        bool exists(int num, std::stack<int> chain){
            // checks if an item already exists in a chain

            // no point checking empty chain
            if (!chain.size()){
                return false;
            }

            while (!chain.empty()){
                int currentNumber = chain.top();
                if (currentNumber == num){
                    return true;
                }
                chain.pop();
            }

            return false;
        }
    public:
        Hash() {
            // initialize table
            bool generating = true;
            int count = 0;
            while (generating) {
                if (count == 999){
                    generating = false;
                }
                int randomNum = rand() % 99;
                // numbers must be in range 10 - 99
                if (randomNum >= 10){
                    // find slot
                    int slot = this->hashFunc(randomNum);
                    std::stack<int> chain = this->table[slot];

                    // check if slot already has the random number 
                    bool exists = this->exists(randomNum,chain);
                    
                    // insert number if it does not exist
                    if (!exists){
                        this->table[slot].push(randomNum);
                    }
                    ++count;
                }
            }
        }
        // prints the table 
        void printTable() {
            for (int slot = 0; slot < SIZE; ++slot){
                std::cout << "[" << (slot + 1) * 10 << "]: [";
                std::stack<int> chain = this->table[slot];
                while (!chain.empty()){
                    if (chain.size() > 1){
                        std::cout << chain.top() << ", ";
                    }
                    else {
                        std::cout << chain.top();
                    }
                    chain.pop();
                }
                std::cout << "]\n";
            }
        }
        
};


int main(){
    Hash hashTable;
    hashTable.printTable();
    return 0;
}