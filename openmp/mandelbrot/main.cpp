#include <iostream>
#include <omp.h>
#include <cstring>
#include <vector>
#include <algorithm>
#include <math.h>
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h" // This has utility functions to save image array to png, Reference: Prof. Daniele Panozzo https://github.com/danielepanozzo/cg/tree/master/ext/stb

using namespace std;

double X_MIN = -2.0;
double X_MAX = 1;
double Y_MIN = -1.5;
double Y_MAX = 1.5;
int IMG_HEIGHT = 4096;
int IMG_WIDTH = 4096;
int MAX_ITERATIONS = 100;
double DIVERGENCE_THRESHOLD = 4.0; // Absolute value after which the mandel set is assumed to diverge

unsigned char double_to_unsignedchar(const double d)
{
    return round(std::max(std::min(1., d), 0.) * 255);
}

void write_matrix_to_uint8(double* data, int h, int w, std::vector<uint8_t> &image)
{
    image.resize(w * h * 4, 0);

    unsigned i = 0, x;
    for (int j=0; j<h*w; j++)
    {
        x = double_to_unsignedchar(data[j]);
        image[i + 0] = x;
        image[i + 1] = 0;
        image[i + 2] = 0;
        image[i + 3] = 255;
        i += 4;
    }
}

void save_img(double* data, int h, int w, char* filename) {
    std::vector<uint8_t> image;
    write_matrix_to_uint8(data, h, w, image);
    stbi_write_png(filename, w, h, 4, image.data(), w*4);
}


double mandel(double c_re, double c_im) {
    int count = 0;
    double z_re = 0.0;
    double z_im = 0.0;
    double z_re_new, z_im_new;
    while (count < MAX_ITERATIONS) {
        if ((z_re*z_re + z_im*z_im) > DIVERGENCE_THRESHOLD) {
            break;
        }
        z_re_new = z_re*z_re - z_im*z_im;
        z_im_new = 2.0 * z_re * z_im;
        z_re = z_re_new + c_re;
        z_im = z_im_new + c_im;
        count += 1;
    }
    return double(count)/double(MAX_ITERATIONS);
}

void mandelbrot_serial(double* out) {
    double start = omp_get_wtime();

    double dx = (X_MAX - X_MIN) / IMG_WIDTH;
    double dy = (Y_MAX - Y_MIN) / IMG_HEIGHT;

    double x, y;
    for (int i = 0; i < IMG_HEIGHT*IMG_WIDTH; i++) {
        x = X_MIN + (i % IMG_WIDTH) * dx;
        y = Y_MIN + (i / IMG_WIDTH) * dy;
        out[i] = mandel(x, y);
    }
    double end = omp_get_wtime();
    printf ("Mandelbrot serial execution time: %fs \n", end-start);
}

void mandelbrot_parallel(double* out, int n_threads) {
    double start = omp_get_wtime();
    double dx = (X_MAX - X_MIN) / IMG_WIDTH;
    double dy = (Y_MAX - Y_MIN) / IMG_HEIGHT;

    double x, y;
    int i;
#pragma omp parallel for num_threads(n_threads) default(none) private(i, x, y) shared(X_MIN, Y_MIN, dx, dy, IMG_HEIGHT, IMG_WIDTH, out)
    for (i = 0; i < IMG_HEIGHT*IMG_WIDTH; i++) {
        x = X_MIN + (i % IMG_WIDTH) * dx;
        y = Y_MIN + (i / IMG_WIDTH) * dy;
        out[i] = mandel(x, y);
    }

    double end = omp_get_wtime();
    printf ("Mandelbrot parallel execution time: %fs \n", end-start);
}



int main() {
    // Tunable parameters
    int n_threads = 10;
    MAX_ITERATIONS = 100;

    double* output = new double[IMG_HEIGHT * IMG_WIDTH];
    memset(output, 0, IMG_HEIGHT * IMG_WIDTH * sizeof(double));

    printf("Running Mandelbrot in serial\n");
    mandelbrot_serial(output);
    save_img(output, IMG_HEIGHT, IMG_WIDTH, (char*)"mandelbrot_omp_serial.png");

    memset(output, 0, IMG_HEIGHT * IMG_WIDTH * sizeof(double));
    cout << "Running Mandelbrot in parallel with " << n_threads << " threads" << endl;
    mandelbrot_parallel(output, n_threads);
    save_img(output, IMG_HEIGHT, IMG_WIDTH,(char*)"mandelbrot_omp_parallel.png");
}