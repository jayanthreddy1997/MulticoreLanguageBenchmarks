#!/bin/bash

# find all the text files in the current directory and save them to an array
path_toFiles="/home/pyc298/Input/"
files=( $(find "$path_toFiles" -type f -name "*.txt") )
numbers=( 1 2 4 8 16 32)

#compile the program
g++ -fopenmp parallel.cpp -o parallel

# run the Rust binary with the list of files as an argument
for f in "${files[@]}"; do
    # with varying number of threads
    for ((i=0; i<${#numbers[@]}; i++)); do
        n=${numbers[i]}
        ./parallel "$n" "$f"
    done
done

#compile sequential part of the program
g++ -fopenmp sequential.cpp -o sequential

for f in "${files[@]}"; do
    # with varying number of threads
    ./sequential "$f"
    
done