library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

adapters_raw <- read_delim(here::here("2020", "raw_data", "day_10.txt"), 
                           delim = "\t", 
                           col_names = "jolt")

# Main --------------------------------------------------------------------

##### Part 1 #####

# create port jolt (0) and device jolts
port_device_jolts <- tibble(jolt = c(0, max(adapters_raw[["jolt"]]) + 3))

adapters_tidy <- adapters_raw %>% 
  bind_rows(port_device_jolts) %>% 
  arrange(jolt) %>% 
  mutate(diff_jolt = jolt - lag(jolt)) 

# get num 1/3 differences
adapters_p1 <- adapters_tidy %>% 
  filter(!is.na(diff_jolt)) %>% 
  group_by(diff_jolt) %>% 
  summarise(n = n()) 

prod(adapters_p1[["n"]])

##### Part 2 #####

adapters_p2 <- adapters_tidy %>% 
  filter(!jolt %in% c(0, max(adapters_raw[["jolt"]]) + 3)) %>% 
  mutate(group = NA_integer_)

# group the adapter jolts into segments of difference 1s
group <- 1

for(i in seq_len(nrow(adapters_p2))){
  
  adapters_p2[["group"]][i] <- group
  
  if(adapters_p2[["diff_jolt"]][i] == 3){
    
    group <- group + 1
    
  }
  
}

# filter out the difference of 3s as adds no possiblities 
# find N of segments of 1s, this will be proportional to number of possibilities
# added for that group
adapters_p2 <- adapters_p2 %>% 
  filter(diff_jolt != 3) %>% 
  group_by(group) %>% 
  summarise(n = n()) 

# create key of n to possibilities
n_to_poss <- vector("integer", max(adapters_p2[["n"]]))

for(i in seq_len(max(adapters_p2[["n"]]))){
  
  if(i == 1){
    
    n_to_poss[i] <- 1
    
  }else{
    
    n_to_poss[i] <- n_to_poss[i - 1] + (i - 1)
    
  }
  
}

adapters_p2 <- adapters_p2 %>% 
  mutate(n_to_poss = n_to_poss[n])

# print options to allow full number 
options("scipen" = 100, "digits" = 4)

prod(adapters_p2[["n_to_poss"]]) 

