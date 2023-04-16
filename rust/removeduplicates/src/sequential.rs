use std::collections::HashSet;
use std::fs;
use std::time::Instant;


fn main() {
    let contents = fs::read_to_string("src/example.txt").expect("Error reading file");
    
    let now = Instant::now();
    let unique_words: HashSet<String> = contents    
    .split_whitespace() // Split contents into words sequentially
    .map(|word| word.to_string())
    .fold(HashSet::new(), |mut acc: HashSet<String>, word| {
        acc.insert(word);
        acc
    });
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
    println!("{:?}", unique_words.len());
    
}
