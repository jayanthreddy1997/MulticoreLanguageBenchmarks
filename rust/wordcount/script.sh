#!/bin/bash

# find all the text files in the current directory and save them to an array
path_toFiles="/home/pyc298/Input/"
files=( $(find "$path_toFiles" -type f -name "*.txt") )
numbers=( 1 2 4 8 16 32)
binaries=( "parallel" "parallelsplit" )
# run the Rust binary with the list of files as an argument
for binary in "${binaries[@]}"; do
    for f in "${files[@]}"; do
        #cargo run --release --bin sequential "$f"
        # with varying number of threads
        for ((i=0; i<${#numbers[@]}; i++)); do
            n=${numbers[i]}
            cargo run --release --bin "$binary" "$n" "$f"
        done
    done
done


for f in "${files[@]}"; do
    #cargo run --release --bin sequential "$f"
    # with varying number of threads
    cargo run --release --bin "sequential" "$f"
    
done