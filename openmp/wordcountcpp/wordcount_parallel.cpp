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

void countWords(const vector<char*>& vec, size_t start, size_t end, unordered_map<string, num_words>& wordFreqMap) {
    while (start <= end) {
        ++wordFreqMap[vec[start]].count;
        start += 1;
    }
}
inline void countWord(const vector<string>& vec, size_t i,unordered_map<string, num_words>& wordFreqMap) {

    ++wordFreqMap[vec[i]].count;
}
void reduce_maps(unordered_map<string, num_words>& map1, unordered_map<string, num_words>& map2) {
    for (pair<const string, num_words>& p : map2) {
       map1[p.first].count += p.second.count;
    }
    // return map1;
}

// #pragma omp declare reduction(reduce_umap : \
//     unordered_map<string, num_words> : \
//     reduce_maps(omp_out, omp_in)) \
//     initializer(omp_priv(omp_orig))

int main(int argv, char** argc) {
    char delimiters[] = " \t\n,.:;";
    ifstream inp;
    inp.open("inp.txt");
    vector<string> vec; //TODO: reserve some capacity for this to avoid overhead when having to expand the vector
    string line;
    //assume that input has no gaps
    while (getline(inp, line)) {
        char* token = strtok(&line[0], delimiters);
        string s = token;
        vec.push_back(token);
        while ((token = strtok(NULL, delimiters))) {
            vec.push_back(token);
        }
    }
    inp.close();
    unordered_map<string, num_words> globalWordFreq;
    omp_set_num_threads(stoi(argc[1]));
    // #pragma omp parallel for private(wordFreq)
    //     reduction(reduce_umap: globalWordFreq)
    // for (size_t i = 0; i < vec.size(); i++) {
    //     countWord(vec, i, wordFreq);

    //     // cout << "\n" << wordFreq.size();
    //     // globalWordFreq += wordFreq;
    // }
    double start_time = omp_get_wtime();
    #pragma omp parallel 
    {
        unordered_map<string, num_words> wordFreq;
        #pragma omp for
        for (size_t i = 0; i < vec.size(); i++) {
            countWord(vec, i, wordFreq);
        }
        #pragma omp critical 
        {
            reduce_maps(globalWordFreq, wordFreq);
        }
    }
    double end_time = omp_get_wtime();
    std::cout << "time taken: " << (end_time - start_time) << "\n";
    // for (pair<const string, num_words>& p : globalWordFreq) {
    //    cout << p.first << ": " << p.second.count << "\n";
    // }
    return 0;
}