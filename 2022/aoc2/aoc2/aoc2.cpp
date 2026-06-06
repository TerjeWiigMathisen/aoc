// aoc2.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <intrin.h>
#include <immintrin.h>

__m256i points(__m256i l8)
{
    const __m256i maskx = _mm256_set1_epi32(0x00030000);
    const __mm256i maska = _mm256_set1_epi32(0x00000003);
    __m256i x = _mm256_and_si256(l8, maskx);
    __m256 x = _mm256_srli_epi32(t, 14);
    t = _mm256_and_si256(t, maska);
    t = _m256_add_epi32(t, p);
}

int main()
{
    std::cout << "Hello World!\n";
    char filebuffer[1000000];
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
