// day22.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <vector>
#include <string>
using namespace std;

uint32_t iterate(uint32_t sec)
{
    sec = ((sec << 6) ^ sec) & 16777215;
    sec = ((sec >> 5) ^ sec) & 16777215;
    sec = ((sec << 11) ^ sec) & 16777215;
    return sec;
}

int64_t process(const vector<string> lines)
{
    vector<uint32_t> numbers;
    for (auto line = lines.begin(); line != lines.end(); line++) {
        unit32_t num = 
        numbers.push_back(stoul(line, nullptr, 10))

    }
}

int main(int argc, char*argv[], char *env[])
{
    int64_t part1 = process("1\n\2\n\3\n2024");
    std::cout << "Hello World!\n";
}

// Run program: Ctrl + F5 or Debug > Start Without Debugging menu
// Debug program: F5 or Debug > Start Debugging menu

// Tips for Getting Started: 
//   1. Use the Solution Explorer window to add/manage files
//   2. Use the Team Explorer window to connect to source control
//   3. Use the Output window to see build output and other messages
//   4. Use the Error List window to view errors
//   5. Go to Project > Add New Item to create new code files, or Project > Add Existing Item to add existing code files to the project
//   6. In the future, to open this project again, go to File > Open > Project and select the .sln file
