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
    //Filename 
    string filename = argv[1];
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

    //start clock
    start = omp_get_wtime();
    for(int i=0; i<words.size(); i++){
        hashset.insert(words[i]);
    }
    end = omp_get_wtime();
    
    // for (const string& word: hashset){
    //      cout<< word<< " \n";
    // }

    cout << "Sequntial ---> " << "File: " << filename;
    cout << "\n" << end-start << "s\n" << hashset.size() << endl;
    
    return 0;
}
