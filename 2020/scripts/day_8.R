library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

boot_raw <- read_delim(here::here("2020", "raw_data", "day_8.txt"), 
                       delim = "\t", 
                       col_names = "command")

# Main --------------------------------------------------------------------

##### Part 1 #####

# tidy to split command + value
boot_tidy <- 
  boot_raw %>% 
  separate("command", 
           c("command", "value"), 
           sep = " ") %>% 
  mutate(value = value %>% as.integer(), 
         index = row_number())

run_boot_code <- function(boot_code, acc, jmp){
  
  # replicate boot code run
  # need to store index his and break while when command 
  # run a second time (i.e. infinite loop)
  acc_total <- 0
  index_his <- c(1)
  index <- 1
  
  while(any(duplicated(index_his)) == FALSE &&
        index %in% boot_code[["index"]]){
    
    command_curr <- boot_code[["command"]][index]
    value_curr <- boot_code[["value"]][index]
    
    if(command_curr == "acc"){
      
      acc_total <- acc(value_curr, acc_total)
      index <- index + 1
      
    }else if(command_curr == "jmp"){
      
      index <- jmp(value_curr, index)
      
    }else if(command_curr == "nop"){
      
      index <- index + 1
      
    }
    
    index_his <- c(index_his, index)
    
  }
  
  boot_result <- 
    tibble(acc_total = acc_total, 
           final_index = index, 
           index_his = list(index_his))
  
  return(boot_result)
  
}

# code acc/jmp commands 
acc <- function(value, acc_total){
  
  acc_total <- acc_total + value
  
  return(acc_total)
  
}

jmp <- function(value, index){
  
  index <- index + value
  
  return(index)
  
}

acc_p1 <- run_boot_code(boot_tidy, acc, jmp)
acc_p1[["acc_total"]]

##### Part 2 #####

# if it's a single nop/jmp change to break the infinite loop
# then it must be within our index his, as we've already hit the loop 
jmp_nop_candidates <- boot_tidy[acc_p1[["index_his"]][[1]], ] %>% 
  filter(command %in% c("nop", "jmp"))

convert_jmp_nop <- function(command){
  
  new_command <- ifelse(command == "jmp", "nop", "jmp")
  
  return(new_command)
  
}

acc_total_all <- vector("list", nrow(jmp_nop_candidates))

for(i in seq_len(nrow(jmp_nop_candidates))){
  
  jmp_nop_curr <- jmp_nop_candidates[i, ]
  boot_curr <- boot_tidy %>% 
    mutate(command = ifelse(index == jmp_nop_curr[["index"]], 
                            convert_jmp_nop(command), 
                            command))
  
  # needed to add a catch for when index is outside of 
  # possible indices in the function above
  acc_total_all[[i]] <- run_boot_code(boot_curr, acc, jmp)
  
}

acc_p2 <- acc_total_all %>% 
  lapply(function(x) x %>% select(-index_his)) %>% 
  do.call(bind_rows, .) %>% 
  filter(final_index == nrow(boot_tidy) + 1)

acc_p2[["acc_total"]]

