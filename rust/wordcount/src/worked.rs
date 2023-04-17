use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;

fn main() {
    let text = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog.";

    //Create threads
    let thread_pool_result = ThreadPoolBuilder::new()
        .num_threads(4) // specify 4 threads
        .build();

    
    let thread_pool = match thread_pool_result {
        Ok(pool) => pool,
        Err(e) => panic!("Failed to create thread pool: {}", e),
    };


    let word_counts = thread_pool.install(|| {
        text.par_split_whitespace()
            .fold(HashMap::new, |mut counts, word| {
                *counts.entry(word).or_insert(0) += 1;
                counts
            })
            .reduce(HashMap::new, |mut map1, map2| {
                for (word, count) in map2 {
                    *map1.entry(word).or_insert(0) += count;
                }
                map1
            })
    });

    println!("{:?}", word_counts);

}
