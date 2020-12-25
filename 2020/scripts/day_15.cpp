#include <Rcpp.h>
using namespace Rcpp;

int get_next_num_cpp(std::map<int, int> his, int num_prev, int i_curr) {
  
  // map.find() will return an iterator to the entry if key present
  // if not, will be the end of map
  std::map<int, int>::iterator it; 
  it = his.find(num_prev);
  
  // if not found, return 0 
  if(it == his.end()) {
    
    return 0; 
    
  }
  
  // if found minus the index of num_prev position 
  // from the 
  int i_prev = it->second; 
    
  return i_curr - 1 - i_prev;
  
}

// [[Rcpp::export]]
int get_n_value_cpp(int n, std::vector<int> start_seq) {
  
  // declare a map to store the values that have appeared
  // key = number, value = index
  std::map<int, int> his; 
  
  // declare outside loop to avoid cost of declaration per iteration
  int num_curr;
  int num_prev;
  
  for(int i = 0; i < n; ++i) {
    
    // store the starting numbers in the map
    if(i < start_seq.size()) {
         
      num_curr = start_seq[i]; 
         
    }else { 
      
      // otherwise calculate and store the next money
      num_curr = get_next_num_cpp(his, num_prev, i);
      
    }
    
    if(i != 0){
      
      
      his[num_prev] = i - 1;
      
    }
    
    num_prev = num_curr;
    
  }

  return num_curr; 
  
}

/*** R
get_n_value_cpp(6, 1:3)
*/
