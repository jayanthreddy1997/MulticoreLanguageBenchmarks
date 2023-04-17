use ndarray::{arr2, Array2, Array1};
use std::thread;
use rayon::prelude::*;
use rand;
use std::time::Instant;

fn matrix_vec_mult_sequential(m: usize, n: usize, A: &Array2<f64>, v: &Vec<f64>, res: &mut Vec<f64>) -> f64 {
    let now = Instant::now();
    for i in 0..m {
        let mut sum: f64 = 0.0;
        for j in 0..n {
            sum += A[[i, j]] * v[j];
        }
        res[i] = sum;
    }
    let elapsed = now.elapsed();
    return elapsed.as_secs_f64();
}

fn matrix_vec_mult_parallel(m: usize, n: usize, A: &Array2<f64>, v: &Vec<f64>, res: &mut Vec<f64>, thread_count: usize) -> f64 {
    rayon::ThreadPoolBuilder::new().num_threads(thread_count).build_global().unwrap();

    let now = Instant::now();
    // TODO
    let elapsed = now.elapsed();
    return elapsed.as_secs_f64();
}

fn get_matrix(m: usize, n: usize, random_init: bool) -> Array2<f64> {
    let mut A: Array2<f64> = Array2::zeros((m, n));

    if random_init {
        for i in 0..m {
            for j in 0..n {
                A[[i, j]] = rand::random();
            }
        }
    }
    return A;
}

fn get_vec(m: usize, random_init: bool) -> Vec<f64> {
    let mut A: Vec<f64> = vec![0.0; m];

    if random_init {
        for i in 0..m {
            A[i] = rand::random();
        }
    }

    return A;
}

fn main() {
    let m: usize = 1024;
    let n: usize = 1024;
    let run_parallel: bool = false;
    let n_threads: usize = 10;

    println!("Initializing matrices.\n");
    let A= get_matrix(m, n, true);
    let v = get_vec(n, true);
    let mut res = get_vec(m, false);

    let run_time: f64;
    if !run_parallel {
        println!("Running Matrix Vector multiplication.\nMatrix Size: {} x {}\nParallel Execution: {}\n",
               m, n, run_parallel);
        run_time = matrix_vec_mult_sequential(m, n, &A, &v, &mut res);
    } else {
        println!("Running Matrix Vector multiplication.\nMatrix Size: {} x {}\nParallel Execution: {}\nThread Count: {}",
               m, n, run_parallel, n_threads);
        run_time = matrix_vec_mult_parallel(m, n, &A, &v, &mut res, n_threads);
    }
    println!("Run time: {}s", run_time);
}