library(tidyverse)
library(stringr)
library(gmp)

# Load data ---------------------------------------------------------------

times_raw <- read_delim(here::here("2020", "raw_data", "day_13.txt"), 
                        delim = "\t", 
                        col_names = "time")

# Main --------------------------------------------------------------------

##### Part 1 #####

timestamp <- times_raw[[1, "time"]] %>% 
  as.integer()

times_tidy <- times_raw %>% 
  .[[2, "time"]] %>% 
  str_split(",") %>% 
  unlist()

times_tidy <- times_tidy[times_tidy != "x"] %>% 
  as.integer()

for(i in seq_along(times_tidy)){
  
  timestamp %% times_tidy[i]
  
}

waiting_times <- abs((timestamp %% times_tidy) - times_tidy)

min_waiting_time <- which.min(waiting_times)

waiting_times[min_waiting_time] * times_tidy[min_waiting_time]

##### Part 2 #####

# after much googling, it looks like this problem is solved via the 
# chinese remainder theorum
# https://www.dave4math.com/mathematics/chinese-remainder-theorem/
times_tidy <- times_raw %>% 
  .[[2, "time"]] %>% 
  str_split(",") %>% 
  unlist()

# let's say t is the time/solution we are looking for
# and e.g. the bus times are 7 and 13, no x's
# then t is a multiple of 7 (t %% 7 = 0)
# and t + 1 must be a multiple of 13 (t + 1 && 13 = 0)
# the latter also means t && 13 = 13 - 1 = 12
# let's calculate the rems
times_tidy <- 
  tibble(mod = times_tidy) %>% 
  mutate(id = row_number() - 1, 
         mod = mod %>% as.integer()) %>% 
  filter(!is.na(mod)) %>% 
  mutate(rem = ifelse(id == 0, id,  mod - (id %% mod)))

get_mod_1_factor <- function(x, mod){
  
  mod_factors <- vector("integer", length(x))
  
  for(i in seq_along(mod_factors)){
    
    if(as.integer((x[i] %% mod[i])) %in% c(0,1)){
      
      mod_factors[i] <- 1
      
      next
      
    }
    
    # need to set this as "big" integers 
    # incase they get too for R
    factor_curr <- 1
    x_big_start <- gmp::as.bigz(x[i])
    x_big_curr <- x_big_start
    
    while(x_big_curr %% mod[i] != 1){
      
      x_big_curr <- x_big_start * factor_curr
      factor_curr <- factor_curr + 1
      
    }
    
    mod_factors[i] <- factor_curr - 1
    
  }
  
  return(mod_factors)
  
}

# need big numbers otherwise calculations lose accuracy
mod <- times_tidy[["mod"]] %>% 
  gmp::as.bigz()

rem <- times_tidy[["rem"]] %>% 
  gmp::as.bigz()

prod <- prod(mod)
prod_div_mod <- prod/mod
mod_1_factor <- get_mod_1_factor(x = prod_div_mod, mod)

sum(rem * prod_div_mod * mod_1_factor) %% prod
