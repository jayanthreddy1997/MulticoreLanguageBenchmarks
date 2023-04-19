
use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::env;


fn main() {
    let filename = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(filename).expect("Error reading file");
    let now = Instant::now();
    let counts = contents    
    .split_whitespace() // Split contents into words sequentially
    .fold(HashMap::new(), |mut map, word| {
        *map.entry(word.to_string()).or_insert(0) += 1;
        map
    });
    let elapsed = now.elapsed();
    println!("Elapsed: {:?}", elapsed);
    println!("{}", counts.keys().len());
}
