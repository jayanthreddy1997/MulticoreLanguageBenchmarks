
use std::collections::HashMap;
use std::fs;
use std::time::Instant;


fn main() {
    let contents = fs::read_to_string("src/example.txt").expect("Error reading file");
    
    let now = Instant::now();
    let counts = contents    
    .split_whitespace() // Split contents into words sequentially
    .fold(HashMap::new(), |mut map, word| {
        *map.entry(word.to_string()).or_insert(0) += 1;
        map
    });
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("{}", counts.keys().len());
}
