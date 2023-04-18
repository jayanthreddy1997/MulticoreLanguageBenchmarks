use std::env;
use std::time::Instant;
fn main() {
    let arg = env::args().nth(1);
    let num_threads = arg.expect("Need 'num_threads' as an argument!").parse::<usize>().ok().expect("num_threads has to be an integer!");
    let now = Instant::now();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    pool.install(|| 3+5);
    let elapsed = now.elapsed();
    println!("Elapsed during creation&destruction: {:.6?}", elapsed);
}
