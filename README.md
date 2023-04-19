# MulticoreLanguageBenchmarks
Performance benchmarks of multiple computer languages in a multicore setting

g++ version: 12.2.0
rust version: 1.68.1

## Run instructions
Note: All tunable parameters like enable/disable parallelism, number of threads etc are hardcoded in first few lines of the main function of each experiment.
- Openmp
  - Mandelbrot </br>
    Run Directory: <code>openmp/mandelbrot</code> </br>
    Compile: <code>g++ -fopenmp -Istb -O3 main.cpp </code> </br>
    Run:
    - Serial: <code> ./a.out s <max_iterations> </code>
      e.g. <code>./a.out s 100</code>
    - Parallel: <code> ./a.out p <n_threads> <max_iterations> </code>
      e.g. <code> ./a.out p 8 100 </code>
  - Matrix Vector multiplication
    Run Directory: <code>openmp/matrix_vector_mult</code> </br>
    Compile: <code>g++ -fopenmp -O3 main.cpp; </code> </br>
    Run: 
    - Serial: <code> ./a.out <n_rows> <n_cols> s </code> </br>
      e.g. <code> ./a.out 8192 8192 s </code>
    - Parallel: <code> ./a.out <n_rows> <n_cols> p <n_threads> </code> </br>
      e.g. <code>./a.out 8192 8192 p 8</code>
  - Stable Sort (Merge Sort)
    Run Directory: <code>openmp/sorting</code> </br>
    Compile: <code>g++ -fopenmp -O3 parallel_sort.cpp </code> </br>
    Run:
    - Serial: <code> ./a.out <input_size> S M </code>
    - Parallel: <code> ./a.out <input_size> P <num_threads> M </code>
  - Unstable Sort (Quick Sort) 
    Run Directory: <code>openmp/sorting</code> </br>
    Compile: <code>g++ -fopenmp -O3 parallel_sort.cpp </code> </br>
    Run:
      - Serial: <code> ./a.out <input_size> S Q </code>
      - Parallel: <code> ./a.out <input_size> P <num_threads> Q </code>
  - Word Count
    Run Directory: <code>openmp/wordcount</code> </br>
    Command: <code>g++ -fopenmp wordcount_SP.cpp </code> </br>
    Run: 
    - Serial: <code> ./a.out <filename> S </code> </br>
      e.g. <code> ./a.out example.txt S </code>
    - Parallel: <code> ./a.out <filename> P <n_threads> </code> </br>
      e.g. <code>./a.out example.txt P 8</code>
  - Remove Duplicates
    Run Directory: <code>openmp/removeduplicates</code> </br>
    Command Parallel: <code>g++ -fopenmp parallel.cpp - o parallel </code> </br>
    Command Sequential: <code>g++ -fopenmp sequential.cpp -o sequential </code> </br>
    Run: 
    - Serial: <code> ./sequential <filename> </code> </br>
      e.g. <code> ./sequential example.txt </code>
    - Parallel: <code> ./parallel <n_threads> <filename> </code> </br>
      e.g. <code>./parallel 8 example.txt </code>
    
- Rust
  - Mandelbrot </br>
    Run Directory: <code>rust/mandelbrot</code> </br>
    Command: 
    - Serial: <code>cargo run -r -- s <max_iterations> </code>
      e.g. <code>cargo run -r -- s 100 </code>
    - Parallel: <code>cargo run -r -- p <n_threads> <max_iterations> </code>
      e.g. <code>cargo run -r -- p 8 100 </code>
  - Matrix Vector multiplication </br>
    Run Directory: <code>rust/matrix_vector_mult</code> </br>
    Command:
    - Serial: <code>cargo run -r -- <n_rows> <n_cols> s </code>
    e.g. <code>cargo run -r -- 1024 1024 s </code>
    - Parallel: <code>cargo run -r -- <n_rows> <n_cols> p <n_threads> </code>
      e.g. <code>cargo run -r -- 1024 1024 p 8 </code>
  - Stable Sort (Merge Sort)
    Run Directory: <code>rust/sorting</code> </br>
    Command:
    - Serial: <code> cargo run -r -- <input_size> S M </code>
    - Parallel: <code> cargo run -r -- <input_size> P <num_threads> M </code>
  - Unstable Sort (Quick Sort)
    Run Directory: <code>rust/sorting</code> </br>
    Command:
    - Serial: <code> cargo run -r -- <input_size> S Q </code>
    - Parallel: <code> cargo run -r -- <input_size> P <num_threads> Q </code>
  - Word Count 
    Run Directory: <code> rust/wordcount</code> </br>
    Command:
    - Serial: <code> cargo run --release --bin sequential <filename> </code>
    - Parallel: <code> cargo run --release --bin parallel <num_threads> <filename> </code> 
  - Remove Duplicates 
    Run Directory: <code> rust/removeduplicates</code> </br>
    Command:
    - Serial: <code> cargo run --release --bin sequential <filename> </code>
    - Parallel: <code> cargo run --release --bin parallel <num_threads> <filename> </code> 

