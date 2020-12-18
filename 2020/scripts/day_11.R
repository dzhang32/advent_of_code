library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

seats_raw <- read_delim(here::here("2020", "raw_data", "day_11.txt"), 
                        delim = "\t", 
                        col_names = "layout")

# Main --------------------------------------------------------------------

##### Part 1 #####

# convert L's to 0 and .'s to NA's
# then convert to matrix
layout_ncol <- str_count(seats_raw[["layout"]][1]) 

seats_tidy <- 
  seats_raw %>% 
  mutate(layout = layout %>% 
           str_replace_all("L", "0")) %>% 
  separate(layout, into = str_c("X", 0:layout_ncol), sep = "") %>% 
  dplyr::select(-X0) %>% 
  mutate_all(as.integer) 

# create function to obtain all adj seats
get_adj_seat <- function(row_col){
  
  adj_seats <- 
    tibble(row = (row_col[["row"]] - 1):(row_col[["row"]] + 1), 
           col = (row_col[["col"]] - 1):(row_col[["col"]] + 1)) %>% 
    expand(row, col) %>% 
    anti_join(row_col, by = c("row", "col"))
  
  return(adj_seats)
  
}

# apply function to each cell 
adj_seats_key <- seats_tidy %>% 
  mutate(row = row_number()) %>% 
  gather(key = "col", value = "seat", contains("X")) %>% 
  dplyr::select(-seat) %>% 
  mutate(col = col %>% 
           str_remove("X") %>% 
           as.integer(), 
         adj_seat = vector("list", n())) 

for(i in seq_len(nrow(adj_seats_key))){
  
  adj_seats_key[["adj_seat"]][[i]] <- get_adj_seat(adj_seats_key[i,])
  
}

seats_mat <- seats_tidy %>% 
  as.matrix()

# create a function to update the seat layout
update_seats <- function(seats_mat, adj_seats_key){
  
  # create fresh mat as we simultaneously update
  seats_mat_next <- matrix(nrow = nrow(seats_mat), 
                           ncol = ncol(seats_mat))
  
  for(i in seq_len(nrow(adj_seats_key))){
  
    seat_occ <- seats_mat[[adj_seats_key[["row"]][i], adj_seats_key[["col"]][i]]]
    
    # skip if not a seat
    if(is.na(seat_occ)){
      
      next
      
    }
    
    adj_seats_occ <- sum_adj_seats(seats_mat, adj_seats_key[["adj_seat"]][[i]])
    
    if(seat_occ == 0 && adj_seats_occ == 0){
      
      seats_mat_next[adj_seats_key[["row"]][i], adj_seats_key[["col"]][i]] <- 1
      
    }else if(seat_occ == 1 && adj_seats_occ >= 4){
      
      seats_mat_next[adj_seats_key[["row"]][i], adj_seats_key[["col"]][i]] <- 0
      
    }else{
      
      seats_mat_next[adj_seats_key[["row"]][i], adj_seats_key[["col"]][i]] <- seat_occ
      
    }
    
  }
  
  return(seats_mat_next)
  
}

sum_adj_seats <- function(seats_mat, adj_seats){
  
  adj_seats_occ <- 0
  
  for(j in seq_len(nrow(adj_seats))){
    
    # check if the seat actually exists (not e.g. row 0, col 0)
    real_seat <- 
      tryCatch(expr = {
        
        seats_mat[[adj_seats[["row"]][j], adj_seats[["col"]][j]]]
        
      }, error = function(x) FALSE)
    
    if(is.na(real_seat) | real_seat == FALSE){
      
      next
      
    }
    
    adj_seats_occ <- adj_seats_occ + real_seat
    
  }
  
  return(adj_seats_occ)
  
}

seats_mat_prev <- NA
seats_mat_curr <- seats_mat

iter <- 1

while(!identical(seats_mat_curr, seats_mat_prev)){
  
  print(iter)
  
  seats_mat_prev <- seats_mat_curr
  seats_mat_curr <- update_seats(seats_mat_prev, adj_seats_key)
  
  iter <- iter + 1
  
}

sum(seats_mat_curr, na.rm = TRUE)

##### Part 2 #####

# create a new function to update the seat layout
update_seats <- function(seats_mat){
  
  # create fresh mat as we simultaneously update
  seats_mat_next <- matrix(nrow = nrow(seats_mat), 
                           ncol = ncol(seats_mat))
  
  for(i in seq_len(nrow(seats_mat))){
    
    for(j in seq_len(ncol(seats_mat))){
      
      seat_occ <- seats_mat[[i, j]]
      
      # skip if not a seat
      if(is.na(seat_occ)){
        
        next
        
      }
      
      adj_seats_occ <- sum_adj_seats(seats_mat, i, j)
      
      if(seat_occ == 0 && adj_seats_occ == 0){
        
        seats_mat_next[i, j] <- 1
        
      }else if(seat_occ == 1 && adj_seats_occ >= 5){
        
        seats_mat_next[i, j] <- 0
        
      }else{
        
        seats_mat_next[i, j] <- seat_occ
        
      }
      
    }
    
  }
  
  return(seats_mat_next)
  
}

# in part 2, this function needs to be more complex
# we will search in all possible directions 
# for the first seat (non-NA value) then add this to get the sum
# this is a real slow and dirty brute force solution 
sum_adj_seats <- function(seats_mat, i, j){
  
  ops <- 
    tibble(row = c("add", "minus", "none"), 
           col = c("add", "minus", "none")) %>% 
    expand(row, col) %>% 
    filter(!(row == "none" & col == "none")) %>% 
    mutate(occ = NA_integer_)
  
  for(k in seq_len(nrow(ops))){
    
    ops_curr <- ops[k, ]
    row_col_curr <- c(row = i, col = j)
    occ_curr <- NA
    
    while(is.na(occ_curr)){
      
      row_col_curr <- update_row_col(row_col_curr, ops_curr)
      
      occ_curr <- 
        tryCatch(expr = {
          
          seats_mat[[row_col_curr["row"], row_col_curr["col"]]]
          
        }, error = function(x) 0)
      
    }
    
    ops[["occ"]][k] <- occ_curr
    
  }
  
  adj_seats_occ <- sum(ops[["occ"]])
  
  return(adj_seats_occ)
  
}

update_row_col <- function(row_col_curr, ops_curr){
  
  for(l in c("row", "col")){
    
    if(ops_curr[[l]] == "add"){
      
      row_col_curr[l] <- row_col_curr[l] + 1
      
    }else if(ops_curr[[l]] == "minus"){
      
      row_col_curr[l] <- row_col_curr[l] - 1
      
    }
    
  }
  
  return(row_col_curr)
  
}

seats_mat_prev <- NA
seats_mat_curr <- seats_mat

iter <- 1

while(!identical(seats_mat_curr, seats_mat_prev)){
  
  print(iter)
  
  seats_mat_prev <- seats_mat_curr
  seats_mat_curr <- update_seats(seats_mat = seats_mat_prev)
  
  iter <- iter + 1
  
}

sum(seats_mat_curr, na.rm = TRUE)
