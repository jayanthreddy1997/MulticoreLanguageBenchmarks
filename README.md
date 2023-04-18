# MulticoreLanguageBenchmarks
Performance benchmarks of multiple computer languages in a multicore setting

## Run instructions
Note: All tunable parameters like enable/disable parallelism, number of threads etc are hardcoded in first few lines of the main function of each experiment.
- Openmp
  - Mandelbrot </br>
    Run Directory: <code>openmp/mandelbrot</code> </br>
    Command: <code>g++ -fopenmp -Istb -O3 main.cpp; ./a.out </code>
  - Matrix Vector multiplication
    Run Directory: <code>openmp/matrix_vector_mult</code> </br>
    Command: <code>g++ -fopenmp -O3 main.cpp; ./a.out </code>
- Rust
  - Mandelbrot </br>
    Run Directory: <code>rust/mandelbrot</code> </br>
    Command: <code>cargo run -r </code>
  - Matrix Vector multiplication </br>
    Run Directory: <code>rust/matrix_vector_mult</code> </br>
    Command: <code>cargo run -r </code>
