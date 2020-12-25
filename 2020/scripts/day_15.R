library(tidyverse)
library(stringr)
library(Rcpp)
library(patchwork)

# Load data ---------------------------------------------------------------

start_num_raw <- read_delim(here::here("2020", "raw_data", "day_15.txt"), 
                            delim = "\t", 
                            col_names = "seq", 
                            col_types = "c")

# Main --------------------------------------------------------------------

##### Part 1 ####

start_num_raw <- 
  start_num_raw[["seq"]] %>% 
  str_split(",") %>% 
  unlist() %>% 
  as.integer()

get_next_num <- function(number_his){
  
  num_curr <- number_his[length(number_his)]
  
  if(sum(number_his %in% num_curr) == 1){
    
    num_next <- 0
    
  }else{
    
    num_next <- which(number_his %in% num_curr)
    num_next <- diff(num_next[(length(num_next) - 1):length(num_next)])
    
  }
  
  number_his <- c(number_his, num_next)
  
  return(number_his)
  
}

get_n_value <- function(n, number_his){
  
  len_to_2020 <- n - length(start_num_raw)
  
  number_his <- start_num_raw
  
  for(i in seq_len(len_to_2020)){
    
    number_his <- get_next_num(number_his)
    
  }
  
  n_value <- number_his[[length(number_his)]]
  
  return(n_value)
  
}

get_n_value(n = 2020, number_his = start_num_raw)

##### Part 2 ####

# the issue here is the implementation above is far too slow to reach 30000000
# there's two possible solutions: 
# 1. rewrite the code using Rcpp - this loop has dependencies on every iteration
# so can't be sped up via vectorisation/parallesation 
# 2. change the implementation - storing only the last index for which each 
# number has appeared instead of the entire sequence
# let's try implementing both
sourceCpp(here::here("2020", "scripts", "day_15.cpp"))

get_n_value(n = 2020, start_num_raw)
get_n_value_cpp(2020, start_num_raw)

# as a test, let's compare how fast the map-based c++ implementation
# compared to the R version across various values of N
bench_results <- bench::press(
  n = c(10, 100, 1000, 10000),
  {
    bench::mark(
      min_iterations = 10,
      R = get_n_value(n, start_num_raw), 
      Cpp = get_n_value_cpp(n, start_num_raw)
    )
  }
)

# much faster and memory efficient
# but more surprisingly R starts to require e.g. ~2Gb of memory for 
# obtaining the 10,000th number
# whilst Cpp only needs 2.49Kb
r_vs_cpp_speed <- bench_results %>% 
  mutate(expression = expression %>% 
           as.character()) %>% 
  ggplot(aes(x = n, 
             y = median, 
             colour = expression)) + 
  geom_line() + 
  geom_point(shape = 4) + 
  scale_x_log10("N iterations") + 
  scale_y_continuous("Median runtime (s)") + 
  theme_bw()

r_vs_cpp_mem <- bench_results %>% 
  mutate(expression = expression %>% 
           as.character(), 
         mem_alloc = mem_alloc %>% 
           as.double(),
         mem_alloc = mem_alloc/1000000) %>% 
  ggplot(aes(x = n, 
             y = mem_alloc, 
             colour = expression)) + 
  geom_line() + 
  geom_point(shape = 4) + 
  scale_x_log10("N iterations") +
  scale_y_continuous("Memory allocation (Mb)") + 
  theme_bw()

r_vs_cpp <- r_vs_cpp_speed + r_vs_cpp_mem

# let's only test Cpp with higher values of N 
bench_results <- bench::press(
  n = c(10, 100, 1000, 10000, 100000, 1000000, 10000000),
  {
    bench::mark(
      max_iterations = 5, 
      Cpp = get_n_value_cpp(n, start_num_raw)
    )
  }
)

# now 10,000,000 iterations takes <5s
bench_results %>% 
  mutate(expression = expression %>% 
           as.character()) %>% 
  ggplot(aes(x = n, 
             y = median, 
             colour = expression)) + 
  geom_line() + 
  geom_point(shape = 4) + 
  scale_x_log10("N iterations") + 
  scale_y_continuous("Median runtime (s)") + 
  theme_bw()

get_n_value_cpp(30000000, start_num_raw)
