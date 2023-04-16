use std::env;
use rand::prelude::*;
use rayon::prelude::*;
use easybench::{bench_env};
fn add_random(list : &mut Vec<i64>, rng : &mut ThreadRng) {
    let y = ((rng.gen::<f64>() * 200.0) - 100.0) as i64;
    list.push(y);
}
fn main() {
    let arg = env::args().nth(1);
    let num = arg.expect("Need 'N' as an argument!").parse::<usize>().ok().expect("N has to be an integer!");
    let num_threads : usize = env::args().nth(2).expect("Please provide number of threads!").parse::<usize>().ok().expect("num_threads has to be an integer");
    let mut vec : Vec<i64> = Vec::with_capacity(num);
    let mut rand_gen = thread_rng();
    for _ in 1..num {
        add_random(&mut vec, &mut rand_gen);
    }
    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    println!("sort:    {}", bench_env(vec, |vec| vec.par_sort_unstable()));
}
