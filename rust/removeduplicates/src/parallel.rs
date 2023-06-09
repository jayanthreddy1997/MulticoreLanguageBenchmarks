use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use std::env;

fn main() {

    let num_threads = env::args().nth(1).unwrap().parse:: <usize>().expect("not a proper string");
    let contents = fs::read_to_string(env::args().nth(2).unwrap()).expect("Error reading file");
    let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let now = Instant::now();
    let unique_words: HashSet<String> = pool
        .install(|| {
            let hashset = contents
                .par_split_whitespace()
                .map(|word| word.to_string())
                .fold(
                    || HashSet::new(),
                    |mut acc: HashSet<String>, word| {
                        acc.insert(word);
                        acc
                    }
                )
                .reduce(HashSet::new, |mut set1, set2|{
                    for word in set2{
                        set1.insert(word);
                    }
                    set1
                });
            hashset
        });
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    println!("{:?}", unique_words.len());
}
