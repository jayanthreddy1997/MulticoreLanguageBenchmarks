use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use std::env;
use std::time::Duration;

fn main() {

    let num_threads = env::args().nth(1).unwrap().parse:: <usize>().expect("not a proper string");
    let filename = env::args().nth(2).unwrap();
    let contents = fs::read_to_string(filename).expect("Error reading file");
    let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    //timing benchmarks
    let mut red_elap: Duration = Duration::new(0,0); 
    let mut fold_elap: Duration = Duration::new(0,0);
    let unique_words: HashSet<String> = pool
        .install(|| {
            let fold_time = Instant::now();
            let parser = contents
                .par_split_whitespace()
                .map(|word| word.to_string());
            let hashset = parser 
                .fold(
                    || HashSet::new(),
                    |mut acc: HashSet<String>, word| {
                        //println!("Thread {} is processing word: {}", rayon::current_thread_index().unwrap(), &word);
                        acc.insert(word);
                        acc
                    }
                );
            fold_elap = fold_time.elapsed();
            let red_time = Instant::now();
            let output = hashset.reduce(HashSet::new, |mut set1, set2|{
                for word in set2{
                    set1.insert(word);
                }
                set1
            });
            red_elap = red_time.elapsed();
            output
        });
    
    println!("Fold Took: {:?}", fold_elap);
    println!("Reduce Took: {:?}", red_elap);

    println!("{:?}", unique_words.len());
}
