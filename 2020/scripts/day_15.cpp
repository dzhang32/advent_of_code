#include <Rcpp.h>
using namespace Rcpp;

// This is a simple example of exporting a C++ function to R. You can
// source this function into an R session using the Rcpp::sourceCpp 
// function (or via the Source button on the editor toolbar). Learn
// more about Rcpp at:
//
//   http://www.rcpp.org/
//   http://adv-r.had.co.nz/Rcpp.html
//   http://gallery.rcpp.org/
//

// [[Rcpp::export]]
int get_next_num_cpp(std::vector<int> seq, int i) {
  
  // store index of matches where his[i] has appeared previously
  std::vector<int> match;
  
  for(int j = 0; j < i; ++j) {
    
    if(seq[i] == seq[j]) {
      
      match.push_back(j);
      
    }
    
  }
  
  // if no matches, next element is 0
  if(match.size() == 0) return 0;
  
  // calculate diff between the current position 
  // and the index of the latest appearence
  int diff = i - match[match.size() - 1];
  
  return diff;
  
}

// [[Rcpp::export]]
int get_n_value_cpp(int n, std::vector<int> his){
  
  // declare a vector of length that we want to obtain
  std::vector<int> seq(n, 0);
  
  // add the values from his into seq
  for(int i = 0; i < his.size(); ++i) {
    
    seq[i] = his[i];
    
  }
  
  // starting from the unknown elements (after his)
  // calculate each next number
  for(int i = his.size(); i < seq.size(); ++i) {
    
    seq[i] = get_next_num_cpp(seq, i - 1);
    
  }
  
  return seq[n - 1];
  
}

// You can include R code blocks in C++ files processed with sourceCpp
// (useful for testing and development). The R code will be automatically 
// run after the compilation.
//

/*** R
get_n_value_cpp(2020, c(1, 3, 2))
*/
