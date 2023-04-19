#include <iostream>
#include <parallel/algorithm>
#include <vector>
#include <fstream>
using namespace std;
int main(int argv, char** argc) {
    int N = stoi(argc[1]);
    char* mode = argc[2];
    int num_threads = 1;
    if (mode[0] == 'P') {
        num_threads = stoi(argc[3]);
    }
    vector<int> vec;
    vec.reserve(N);
    srand(omp_get_wtime());
    for (int i = 0; i < N; i++) {
        vec.push_back(rand() % 201 - 100);// Generate a random number between -100 and 100
    }
    if (mode[0] == 'P') {
        double start = omp_get_wtime();
        omp_set_num_threads(num_threads);
        __gnu_parallel::sort(vec.begin(), vec.end(), __gnu_parallel::quicksort_tag());
        double end = omp_get_wtime();
        cout << (end - start) << "s" << "\n";
    } else {
        double start= omp_get_wtime();
        std::sort(vec.begin(), vec.end());
        double end = omp_get_wtime();
        cout << (end - start) << "s" << "\n";
    }
}