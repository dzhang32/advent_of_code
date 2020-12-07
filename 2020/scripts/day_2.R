library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

passwords_raw <- read_delim(here::here("2020", "raw_data", "day_2.txt"), 
                            delim = " ", 
                            col_names = c("min_max", "letter", "pw"))

# Main --------------------------------------------------------------------

##### Part 1 #####

# tidy the data up, most importantly splitting the max and min
passwords_tidy <- passwords_raw %>% 
  separate(min_max, c("min", "max"), "-") %>% 
  mutate(min = as.numeric(min), 
         max = as.numeric(max), 
         letter = letter %>% 
           str_replace(":", ""))

# count number of those letters 
# then check whether they are >= or <= the min or max
valid <- passwords_tidy %>% 
  mutate(count = str_count(pw, letter)) %>% 
  filter(count >= min & count <= max)

nrow(valid)

##### Part 2 ##### 

# locate the position of the letters in the pws
passwords_p2 <- passwords_tidy %>% 
  mutate(count = str_locate_all(pw, letter))

# check how many of the position rules
# match the actual position of letters in password
matches <- vector("numeric", length = nrow(passwords_p2))

for(i in seq_len(nrow(passwords_p2))){
  
  pos_rule <- c(passwords_p2[["min"]][i], 
                passwords_p2[["max"]][i])
  
  matches[i] <- sum(pos_rule %in% passwords_p2[["count"]][[i]][, "start"])
  
}

sum(matches == 1)
