#include <iostream>
#include <omp.h>

using namespace std;

void matrix_vec_mult_sequential(int m, int n, float** A, float* v, float* res) {
    double start = omp_get_wtime();
    for (int i=0; i<m; i++) {
        res[i] = 0.0;
        for (int j=0; j<n; j++) {
            res[i] += A[i][j] * v[n];
        }
    }
    double end = omp_get_wtime();
    printf("Serial matrix vector multiplication run time: %fs\n", (end-start));
}

void matrix_vec_mult_parallel(int m, int n, float** A, float* v, float* res, int thread_count) {
    int i, j;
    double start = omp_get_wtime();
#pragma omp parallel for num_threads(thread_count) \
        default(none) private(i, j) shared(A, v, res, m, n)
    for (i=0; i<m; i++) {
        res[i] = 0.0;
        for (j=0; j<n; j++) {
            res[i] += A[i][j] * v[n];
        }
    }
    double end = omp_get_wtime();
    printf("Parallel matrix vector multiplication run time: %fs\n", (end-start));
}

float** get_matrix(int m, int n, bool random_init=false) {
    float** mat = new float*[m];
    for (int i=0; i<m; i++) {
        mat[i] = new float[n];
    }
    if (random_init) {
        for (int i=0; i<m; i++) {
            for (int j=0; j<n; j++) {
                mat[i][j] = (float)(rand()) / (float)(RAND_MAX);
            }
        }
    }
    return mat;
}

float* get_vector(int n, bool random_init=false) {
    float* vec = new float[n];

    if (random_init) {
        for (int i=0; i<n; i++) {
                vec[i] = (float)(rand()) / (float)(RAND_MAX);
        }
    }
    return vec;
}


int main() {
    // m*n Matrix multiplication with n*1 vector
    int m = 8192;
    int n = 8192;
    bool run_parallel = true;
    int n_threads = 10;

    printf("Matrix Size: %d x %d\n Initializing matrices.\n", m, n);
    float** A = get_matrix(m, n, true);
    float* v = get_vector(n, true);
    float* res = get_vector(m, false);

    if (!run_parallel) {
        printf("Running Serial Matrix Vector multiplication.\n");
        matrix_vec_mult_sequential(m, n, A, v, res);
    } else {
        printf("Running Parallel Matrix Vector multiplication with %d threads\n", n_threads);
        matrix_vec_mult_parallel(m, n, A, v, res, n_threads);
    }

    return 0;
}