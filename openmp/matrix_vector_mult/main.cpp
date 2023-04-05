#include <iostream>

using namespace std;

void matrix_vec_mult_sequential(int m, int n, float** A, float** v, float** res) {
    for (int i=0; i<m; i++) {
        res[i][0] = 0.0;
        for (int j=0; j<n; j++) {
            res[i][0] += A[i][j] * v[n][0];
        }
    }
}

void matrix_vec_mult_parallel(int m, int n, float** A, float** v, float** res, int thread_count) {
    int i, j;
#pragma omp parallel for num_threads(thread_count) \
        default(none) private(i, j) shared(A, v, res, m, n)
    for (i=0; i<m; i++) {
        res[i][0] = 0.0;
        for (j=0; j<n; j++) {
            res[i][0] += A[i][j] * v[n][0];
        }
    }
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

int main() {
    // m*n Matrix multiplication with n*1 vector
    int m = 1000;
    int n = 1000;
    bool run_parallel = false;
    int n_threads = 10;

    printf("Initializing matrices.\n");
    float** A = get_matrix(m, n, true);
    float** v = get_matrix(n, 1, true);
    float** res = get_matrix(m, 1, false);

    printf("Running Matrix Vector multiplication.\nMatrix Size: %d x %d\nParallel Execution: %d\nThread Count: %d\n",
           m, n, run_parallel, run_parallel?n_threads:1);

    run_parallel ? matrix_vec_mult_parallel(m, n, A, v, res, n_threads) : matrix_vec_mult_sequential(m, n, A, v, res);

    return 0;
}