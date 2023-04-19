#include <stdio.h>
#include <string.h>
#include <fstream>
#include <iostream>
#include <unordered_set>
#include <vector>
#include <algorithm>
#include <omp.h>
using namespace std;
int main(int argc, char** argv){
    //# of threads
    int numThreads = (unsigned int) atoi(argv[1]);
    //Filename 
    string filename = argv[2];
    //get the contents from the file
    ifstream contents (filename);
    string line;
    double start;
    double end;
    vector<string> words;
    if (contents.is_open()){
        while(contents >> line){
            words.push_back(line);
        }
    }
    contents.close();
    
    unordered_set<string> hashset = {};
    //Declare custom reduction operator
    #pragma omp declare reduction (                             \
                intersect:                                      \
                std::unordered_set<std::string> :               \
                omp_out.insert(omp_in.begin(), omp_in.end())    \
                )                                               \
        initializer(omp_priv = std::unordered_set<std::string>())
    
    //start clock
    start = omp_get_wtime();
    #pragma omp parallel for num_threads(numThreads) reduction(intersect: hashset)
    for(int i=0; i<words.size(); i++){
        hashset.insert(words[i]);
    }
    end = omp_get_wtime();
    
    // for (const string& word: hashset){
    //      cout<< word<< " \n";
    // }
    cout << "Parallel --> " << "Num Threads: " << numThreads << "File: " << filename;
    cout << "\n" << end-start << "s\n" << hashset.size() << endl;
    
    return 0;
}
