use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("example.txt").expect("Error reading file");

    let counts = contents
        .par_split_whitespace() // Split contents into words in parallel
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

    println!("{:#?}", counts);
}
