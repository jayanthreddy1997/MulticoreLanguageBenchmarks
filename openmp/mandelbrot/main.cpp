#include <iostream>
#include <omp.h>
#include <cstring>
#include <vector>
#include <algorithm>
#include <math.h>
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

using namespace std;

double X_MIN = -2.0;
double X_MAX = 1;
double Y_MIN = -1.5;
double Y_MAX = 1.5;
int IMG_HEIGHT = 1024;
int IMG_WIDTH = 1024;
int MAX_ITERATIONS = 100;
double COMPUTATION_LIMIT = 4.0; // Absolute value after which the mandel set is assumed to diverge

unsigned char double_to_unsignedchar(const double d)
{
    return round(std::max(std::min(1., d), 0.) * 255);
}

void write_matrix_to_uint8(int* data, int h, int w, std::vector<uint8_t> &image)
{
    image.resize(w * h * 4, 0);

    int loc;
    unsigned i = 0;
    for (unsigned hi = 0; hi < h; ++hi)
    {
        for (unsigned wi = 0; wi < w; ++wi)
        {
            loc = (hi * w * 4) + (wi * 4);
            image[loc + 0] = double_to_unsignedchar(data[i+0]);
            image[loc + 1] = double_to_unsignedchar(data[i+1]);
            image[loc + 2] = double_to_unsignedchar(data[i+2]);
            image[loc + 3] = double_to_unsignedchar(1.0);
            i += 3;
        }
    }
}

void save_img(int* data, int h, int w, char* filename) {
    std::vector<uint8_t> image;
    write_matrix_to_uint8(data, h, w, image);

    stbi_write_png(filename, w, h, 4, image.data(), w*4);
}


void mandelbrot_serial(int* out) {
    double start = omp_get_wtime();
//    float dx = (x1 - x0) / width;
//    float dy = (y1 - y0) / height;
//
//    for (int j = 0; j < height; j++) {
//        for (int i = 0; i < width; ++i) {
//            float x = x0 + i * dx;
//            float y = y0 + j * dy;
//            int index = (j * width + i);
//            output[index] = mandel(x, y, maxIterations);
//        }
//    }
    double end = omp_get_wtime();
    printf ("Mandelbrot serial execution time: %fs \n", end-start);
}

void mandelbrot_parallel(int n_threads) {
    double start = omp_get_wtime();

    double end = omp_get_wtime();
    printf ("Mandelbrot parallel execution time: %fs \n", end-start);
}



int main() {
    // Tunable parameters
    int n_threads = 10;
    MAX_ITERATIONS = 100;

    int* output = new int[IMG_HEIGHT * IMG_WIDTH];
    memset(output, 0, IMG_HEIGHT * IMG_WIDTH * sizeof(int));

    cout << "Running Mandelbrot in serial" << endl;
    mandelbrot_serial(output);
    save_img(output, IMG_HEIGHT, IMG_WIDTH, "mandelbrot_serial.png");

//    memset(output, 0, IMG_HEIGHT * IMG_WIDTH * sizeof(int));
//    cout << "Running Mandelbrot in parallel with " << n_threads << " threads" << endl;
//    mandelbrot_parallel(n_threads);
//    save_img(output, "mandelbrot_parallel.png");
}