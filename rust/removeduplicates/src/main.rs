use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
//gittrials
fn main() {
    let contents = fs::read_to_string("src/example.txt").expect("Error reading file");
    let pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();
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
