#include <iostream>
#include <parallel/algorithm>
#include <vector>
#include <fstream>
using namespace std;
int main(int argv, char** argc) {
    int N = stoi(argc[1]);
    int num_threads = stoi(argc[2]);
    const char* filename = argc[3];
    ifstream inp(filename);
    vector<int> vec;
    vec.reserve(N);
    int i;
    while (inp >> i) {
        vec.push_back(i);
    }
    omp_set_num_threads(num_threads);
    __gnu_parallel::sort(vec.begin(), vec.end(), __gnu_parallel::quicksort_tag());
}