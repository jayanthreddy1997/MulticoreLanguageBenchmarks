use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::env;
use std::time::Duration;
fn main() {
    let num_threads = env::args().nth(1).unwrap().parse:: <usize>().expect("not a proper string");
    let filename = env::args().nth(2).unwrap();

    //read the contents of the file as a string
    let contents = fs::read_to_string(filename).expect("Error reading file");

    //setup the number of threads you want to use
    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();

    //timing benchmarks
    let mut red_elap: Duration = Duration::new(0,0); 
    let mut fold_elap: Duration = Duration::new(0,0);

    //use the threadpool for the following commands
    let counts = thread_pool.install(|| {
        let fold_time = Instant::now();
        let parser = contents
            .par_split_whitespace();
        let hashmap = parser
            .fold(HashMap::new, |mut map, word| {
                *map.entry(word.to_string()).or_insert(0) += 1;
                map
            });
        fold_elap = fold_time.elapsed();
        let red_time = Instant::now();
        let output = hashmap
            .reduce(HashMap::new, |mut map1, map2| {
                for (key, value) in map2 {
                    *map1.entry(key).or_insert(0) += value;
                }
                map1
            });
        red_elap = red_time.elapsed();
        output
    });

    println!("Fold Took: {:?}", fold_elap);
    println!("Reduce Took: {:?}", red_elap);

    println!("{:#?}", counts.keys().len());
}
