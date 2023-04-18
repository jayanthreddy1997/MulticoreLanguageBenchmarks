use std::env;
use rand::prelude::*;
use std::time::Instant;
fn add_random(list : &mut Vec<i64>, rng : &mut ThreadRng) {
    let y = ((rng.gen::<f64>() * 200.0) - 100.0) as i64;
    list.push(y);
}
fn main() {
    let arg = env::args().nth(1);
    let num = arg.expect("Need 'N' as an argument!").parse::<usize>().ok().expect("N has to be an integer!");
    let mut vec : Vec<i64> = Vec::with_capacity(num);
    let mut rand_gen = thread_rng();
    for _ in 1..num {
        add_random(&mut vec, &mut rand_gen);
    }
    let now = Instant::now();
    vec.sort();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.6?}", elapsed);
}
