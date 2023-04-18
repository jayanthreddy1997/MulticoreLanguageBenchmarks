use rayon::prelude::*;
use rand;
use std::time::Instant;
use std::env;

fn matrix_vec_mult_sequential(m: usize, n: usize, A: &Vec<Vec<f64>>, v: &Vec<f64>) -> Vec<f64> {
    let now = Instant::now();

    let res = A.iter()
        .map(|row| {
            let mut sum: f64 = 0.0;
            for i in 0..n {
                sum += row[i] * v[i];
            }
            sum
        })
        .collect::<Vec<f64>>();

    let elapsed = now.elapsed();
    println!("Multiplication run time: {}s", elapsed.as_secs_f64());

    return res;
}

fn matrix_vec_mult_parallel(m: usize, n: usize, A: &Vec<Vec<f64>>, v: &Vec<f64>, thread_count: usize) -> Vec<f64> {
    rayon::ThreadPoolBuilder::new().num_threads(thread_count).build_global().unwrap();

    let now = Instant::now();

    let res = A.par_iter()
        .map(|row| {
            let mut sum: f64 = 0.0;
            for i in 0..n {
                sum += row[i] * v[i];
            }
            sum
        })
        .collect::<Vec<f64>>();

    let elapsed = now.elapsed();
    println!("Multiplication run time: {}s", elapsed.as_secs_f64());
    return res;
}

fn get_matrix(m: usize, n: usize, random_init: bool) -> Vec<Vec<f64>> {
    let mut A: Vec<Vec<f64>> = vec![vec![0.0; n]; m];

    if random_init {
        for i in 0..m {
            for j in 0..n {
                A[i][j] = rand::random();
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
    let args: Vec<String> = env::args().collect();

    let mut m: usize = 8192;
    let mut n: usize = 8192;
    let mut run_parallel: bool = true;
    let mut n_threads: usize = 10;

    if !args.is_empty() {
        m = (&args[1]).parse::<usize>().unwrap();
        n = (&args[2]).parse::<usize>().unwrap();
        run_parallel = (&args[3]).parse::<char>().unwrap()=='p';
        if run_parallel {
            n_threads = (&args[4]).parse::<usize>().unwrap();
        }
    }

    println!("Initializing matrices.\n");
    let A= get_matrix(m, n, true);
    let v = get_vec(n, true);
    let mut res;

    if !run_parallel {
        println!("Running Matrix Vector multiplication.\nMatrix Size: {} x {}\nParallel Execution: {}\n",
               m, n, run_parallel);
        res = matrix_vec_mult_sequential(m, n, &A, &v);
    } else {
        println!("Running Matrix Vector multiplication.\nMatrix Size: {} x {}\nParallel Execution: {}\nThread Count: {}",
               m, n, run_parallel, n_threads);
        res = matrix_vec_mult_parallel(m, n, &A, &v, n_threads);
    }
}