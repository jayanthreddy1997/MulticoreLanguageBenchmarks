#!/bin/bash

# find all the text files in the current directory and save them to an array
files=( $(find . -type f -name "*.txt") )
numbers=( 1 2 4 8 )
binaries=( "parallel" "sequential" )
# run the Rust binary with the list of files as an argument
for binary in "${binaries[@]}"; do
    for ((i=0; i<${#numbers[@]}; i++)); do
        n=${numbers[i]} 
        for f in "${files[@]}"; do 
            cargo run --release --bin "$binary" "$n" "$f"
        done
    done
done

