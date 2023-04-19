#include <iostream>
#include <parallel/algorithm>
#include <vector>
#include <fstream>

using namespace std;

int main(int argc, char** argv) {
    int N = stoi(argv[1]);
    char* mode = argv[2];
    char* sort_mode = (char*)"Q";
    int num_threads = 1;
    if (mode[0] == 'P') {
        num_threads = stoi(argv[3]);
        sort_mode = argv[4];
    } else {
        sort_mode = argv[3];
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
        if (sort_mode[0] == 'Q') {
            __gnu_parallel::sort(vec.begin(), vec.end(), __gnu_parallel::quicksort_tag());
        } else {
            __gnu_parallel::sort(vec.begin(), vec.end(), __gnu_parallel::multiway_mergesort_tag());
        }
        double end = omp_get_wtime();
        cout << (end - start) << "s" << "\n";
    } else {
        double start= omp_get_wtime();
        std::sort(vec.begin(), vec.end());
        double end = omp_get_wtime();
        cout << (end - start) << "s" << "\n";
    }
}