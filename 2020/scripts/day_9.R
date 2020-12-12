library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

codes_raw <- read_delim(here::here("2020", "raw_data", "day_9.txt"), 
                        delim = "\t", 
                        col_names = "code")

# Main --------------------------------------------------------------------

##### Part 1 #####

# starting from the first post preamble number
start_index <- 26
num_p1 <- NA_integer_
sum_nums <- NA_integer_

# break loop when current num not part of possible sums between 
# 2 of the last 25
while(num_p1 %in% sum_nums){
  
  num_p1 <- codes_raw[["code"]][[start_index]]
  
  # obtain all possible sums of past 25 num
  sum_nums <- codes_raw %>% 
    filter(row_number() %in% (start_index - 1):(start_index -25)) %>% 
    mutate(code_2 = code) %>% 
    expand(code, code_2) %>% 
    filter(code != code_2) %>% 
    mutate(sum_nums = code + code_2) %>% 
    .[["sum_nums"]]
  
  start_index <- start_index + 1
  
}

##### Part 2 #####

# start from end, fewer possibilities
start_index <- nrow(codes_raw)
num_p2 <- 0

# while we have not gotten a match with our exception from part 1
# we cycle through each num from the bottom
while(num_p2 != num_p1){
  
  seq_num <- 1
  num_p2 <- 0
  
  # we keep summing the rows at the bottom until 
  # they are equal or greater than exception
  while(num_p2 < num_p1){
    
    indexes_to_sum <- start_index:(start_index - seq_num)
    num_p2 <- sum(codes_raw[["code"]][indexes_to_sum])
    
    seq_num <- seq_num + 1
    
  }
  
  start_index <- start_index - 1
  
}

encrypt_weak <- codes_raw[indexes_to_sum, ]
min(encrypt_weak[["code"]]) + max(encrypt_weak[["code"]])

