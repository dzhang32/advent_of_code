library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

seats_raw <- read_delim(here::here("2020", "raw_data", "day_5.txt"), 
                        delim = "\t", 
                        col_names = c("seat_code"))

# Main --------------------------------------------------------------------

##### Part 1 #####

# let's split the row and col codes
# then normalise to 1 and 0s
# 1 being the front, 0 the back
seats_tidy <- seats_raw %>% 
  mutate(row_code = seat_code %>% 
           str_extract("^.......") %>% 
           str_replace_all("F", "1") %>% 
           str_replace_all("B", "0"),
         col_code = seat_code %>% 
           str_extract("...$") %>% 
           str_replace_all("L", "1") %>% 
           str_replace_all("R", "0"))

decode <- function(codes, max_seat, min_seat = 1){
  
  seat_nums <- vector("integer", length = length(codes))
  
  for(i in seq_along(codes)){
    
    code <- codes[i]
    
    # store current min/max seat
    min_seat_curr <- min_seat
    max_seat_curr <- max_seat
    
    # extract each element of the code
    # then change the seat nums to find actual seat
    # needs floor() for front
    # and ceiling() for back
    for(j in seq_len(str_count(code))){
      
      code_curr <- code %>% str_sub(j, j)
      
      # take the mean/midpoint
      mean_seat_curr <- mean(c(max_seat_curr, min_seat_curr))
      
      if(code_curr == "1"){
        
        max_seat_curr <- floor(mean_seat_curr) 
        
      }else if(code_curr == "0"){
        
        min_seat_curr <- ceiling(mean_seat_curr)
        
      }
      
    }
    
    stopifnot(min_seat_curr == max_seat_curr)
    
    seat_nums[i] <- min_seat_curr
    
  }
  
  return(seat_nums)
  
}

# decode the seat codes, need to -1 
# as we used 1-128 rather than 0-127
seats_p1 <- seats_tidy %>% 
  mutate(row_seat = decode(row_code, max_seat = 128), 
         col_seat = decode(col_code, max_seat = 8), 
         row_seat = row_seat - 1, 
         col_seat = col_seat - 1, 
         seat_id = row_seat * 8 + col_seat)

max(seats_p1[["seat_id"]])

##### Part 2 #####

# create list of all possible seats using expand
seats_all <- seats_p1 %>% 
  expand(row_seat, col_seat) %>% 
  arrange(row_seat, col_seat)

# find empty seats and remove those from the very start/end
my_seat <- seats_all %>% 
  anti_join(seats_p1) %>% 
  filter(row_seat != min(row_seat), 
         row_seat != max(row_seat)) %>% 
  mutate(seat_id = row_seat * 8 + col_seat) 

my_seat[["seat_id"]]
