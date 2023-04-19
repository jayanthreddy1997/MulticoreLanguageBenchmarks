#include <unordered_map>
#include <fstream>
#include <string>
#include <iostream>
#include <vector>
#include <stdio.h>
#include <cstring>
#include <omp.h>
using namespace std;

//can reason that this benchmark is testing how they do when reading from same source.
struct num_words {
    long count = 0; //using a struct for the default value
};

inline void countWord(const vector<string>& vec, size_t i,unordered_map<string, num_words>& wordFreqMap) {

    ++wordFreqMap[vec[i]].count;
}

#pragma omp declare reduction(reduce_umap : \
    unordered_map<string, num_words> : \
    omp_out.insert(omp_in.begin(), omp_in.end())) \
    initializer(omp_priv = unordered_map<string, num_words>())

int main(int argv, char** argc) {
    char delimiters[] = " \t\n,.:;";
    ifstream inp;
    inp.open(argc[1]);
    vector<string> vec; //TODO: reserve some capacity for this to avoid overhead when having to expand the vector
    string line;
    //assume that input has no gaps
    while (getline(inp, line)) {
        char* token = strtok(&line[0], delimiters);
        if (token == 0) {
            continue;
        }
        string s = token;
        vec.push_back(token);
        while ((token = strtok(NULL, delimiters))) {
            vec.push_back(token);
        }
    }
    inp.close();
    unordered_map<string, num_words> wordFreq;
    char* mode = argc[2];
    double start_time, end_time;
    if (mode[0] == 'S') {
        start_time = omp_get_wtime();
        for (size_t i = 0; i < vec.size(); i++) {
            countWord(vec, i, wordFreq);
        }
        end_time = omp_get_wtime();
    } else {
        start_time = omp_get_wtime();
        omp_set_num_threads(stoi(argc[3]));
        #pragma omp parallel for \
            reduction(reduce_umap: wordFreq)
        for (size_t i = 0; i < vec.size(); i++) {
            countWord(vec, i, wordFreq);
        }
        end_time = omp_get_wtime();
    }
    std::cout << "time taken: " << (end_time - start_time) << "\n";
    return 0;
}