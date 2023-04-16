use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


fn main() {
    //read the contents of the file as a string
    let contents = fs::read_to_string("src/example.txt").expect("Error reading file");
    //setup the number of threads you want to use
    let thread_pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();
    //use the threadpool for the following commands
    let now = Instant::now();
    let counts = thread_pool.install(|| {
        let hashmap = contents
            .par_split_whitespace()

            .fold(HashMap::new, |mut map, word| {
                *map.entry(word.to_string()).or_insert(0) += 1;
                map
            })

            .reduce(HashMap::new, |mut map1, map2| {
                for (key, value) in map2 {
                    *map1.entry(key).or_insert(0) += value;
                }
                map1
            });
        hashmap
    });
    let elapsed = now.elapsed(); 
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:#?}", counts.keys().len());
    println!("The type of counts {}", type_of(counts));
    
    
}