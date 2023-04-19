#include <string>
#include <iostream>
#include <omp.h>
int main(int argv, char** argc) {
    int N = std::stoi(argc[1]);
    double current_time = omp_get_wtime();
    #pragma omp parallel num_threads(N)
    {
        3+5;
    }
    double end_time = omp_get_wtime();
    std::cout << (end_time - current_time) << "seconds";

}