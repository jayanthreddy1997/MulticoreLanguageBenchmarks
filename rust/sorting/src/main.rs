use std::env;
use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

fn add_random(list : &mut Vec<i64>, rng : &mut ThreadRng) {
    let y = ((rng.gen::<f64>() * 200.0) - 100.0) as i64;
    list.push(y);
}

fn main() {
    let arg = env::args().nth(1);
    let num = arg.expect("Need 'N' as an argument!").parse::<usize>().ok().expect("N has to be an integer!");
    let mode = env::args().nth(2).expect("Please enter 'S' or 'P' for mode.").parse::<char>().expect("Should be a single character for mode! S or P.");
    let mut num_threads : usize = 1;
    let sort_mode: char; // Q/M (Q-Quicksort M-Mergesort)
    if mode == 'P' {
        num_threads = env::args().nth(3).expect("Please provide number of threads!").parse::<usize>().ok().expect("num_threads has to be an integer");
        sort_mode = env::args().nth(4).expect("Please provide sort mode").parse::<char>().ok().expect("sort_mode has to be Q(Quicksort)/M(Mergesort)");
    } else {
        sort_mode = env::args().nth(3).expect("Please provide sort mode").parse::<char>().ok().expect("sort_mode has to be Q(Quicksort)/M(Mergesort)");
    }
    let mut vec : Vec<i64> = Vec::with_capacity(num);
    let mut rand_gen = thread_rng();
    for _ in 1..num {
        add_random(&mut vec, &mut rand_gen);
    }
    let now = Instant::now();
    if mode == 'P' {
        rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
        if sort_mode == 'Q' {
            vec.par_sort_unstable();
        } else {
            vec.par_sort();
        }
    } else {
        if sort_mode == 'Q' {
            vec.sort_unstable();
        } else {
            vec.sort();
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.6?}", elapsed);
}
