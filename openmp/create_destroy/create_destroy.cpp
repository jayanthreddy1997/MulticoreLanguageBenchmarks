#include <string>
#include <iostream>
#include <omp.h>

int main(int argc, char** argv) {
    int N = std::stoi(argv[1]);
    double current_time = omp_get_wtime();
    #pragma omp parallel num_threads(N)
    {
        int x = 3+5;
    }
    double end_time = omp_get_wtime();
    std::cout << (end_time - current_time) << "seconds";

}