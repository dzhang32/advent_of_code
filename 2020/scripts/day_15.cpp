#include <Rcpp.h>
using namespace Rcpp;

// [[Rcpp::export]]
int get_n_value_cpp(int n, std::vector<int> start_seq) {
  
  // declare outside loop to avoid cost of declaration per iteration
  // this actually creates a huge overhead cost for many iterations 

  // declare a map to store the values that have appeared
  // key = number, value = index
  std::map<int, int> his; 
  std::map<int, int>::iterator it; 
  int num_curr;
  int num_prev;
  
  for(int i = 0; i < n; ++i) {
    
    // store the starting numbers in the map
    if(i < start_seq.size()) {
         
      num_curr = start_seq[i]; 
         
    }else { 
      
      // map.find() will return an iterator to the entry if key present
      // if not, will be the end of map
      it = his.find(num_prev);
      
      if(it == his.end()) {
        
        // if not found, return 0 
        num_curr = 0; 
        
      }else { 
        
        // otherwise minus the the stored index 
        // from the current index
        num_curr = i - 1 - it->second;
        
      }
      
    }
    
    if(i != 0){
      
      // need to store the previous number on the iteration after
      // as get_next_num_cpp calculation relies on previous index
      his[num_prev] = i - 1;
      
    }
    
    num_prev = num_curr;
    
  }

  return num_curr; 
  
}

/*** R
get_n_value_cpp(5, 1:3)
*/
