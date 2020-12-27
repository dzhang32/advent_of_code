library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

grid_raw <- read_delim(here::here("2020", "raw_data", "day_17.txt"),
                       delim = "\t", 
                       col_names = "grid")

# Main --------------------------------------------------------------------

##### Part 1 #####

n_rows <- grid_raw[["grid"]] %>% 
  str_count(".") %>% 
  unique()

grid_tidy <- grid_raw %>% 
  mutate(grid = grid %>% 
           str_replace_all("\\.", "0") %>% 
           str_replace_all("#", "1"), 
         y = row_number()) %>% 
  separate(grid, str_c("X", 0:n_rows), "") %>% 
  dplyr::select(-X0) %>% 
  gather("x", "on", starts_with("X")) %>% 
  mutate(x = x %>% 
           str_replace("X", "") %>% 
           as.integer(), 
         z = 0L, 
         on = on %>% 
           as.integer() %>% 
           as.logical()) %>% 
  dplyr::select(x, y, z, on)

min_max_diff_1 <- function(x){
  
  x <- c(min(x) - 1, 
         max(x) + 1, 
         unique(x))
  
  return(x)
  
}

cycle_p1 <- function(grid){
  
  # add the elements of inifite grid that are relevant
  grid_next <- 
    expand_grid(x = min_max_diff_1(grid[["x"]]), 
                y = min_max_diff_1(grid[["y"]]), 
                z = min_max_diff_1(grid[["z"]])) %>% 
    left_join(grid, 
              by = c("x", "y", "z")) %>% 
    mutate(on = ifelse(is.na(on), FALSE, on), 
           sum_neighbour = NA_integer_)
  
  for(i in seq_len(nrow(grid_next))){
    
    cell <- grid_next[i, ]
    neighbours <- expand_grid(x = min_max_diff_1(cell[["x"]]), 
                              y = min_max_diff_1(cell[["y"]]), 
                              z = min_max_diff_1(cell[["z"]])) %>% 
      inner_join(grid_next, 
                 by = c("x", "y", "z")) %>% 
      anti_join(cell, 
                by = c("x", "y", "z"))
    
    grid_next[["sum_neighbour"]][i] <- neighbours[["on"]] %>% sum()
    
  }
  
  grid_next <- grid_next %>% 
    mutate(on = case_when(on & sum_neighbour %in% c(2, 3) ~ TRUE, 
                          !on & sum_neighbour == 3 ~ TRUE, 
                          TRUE ~ FALSE)) %>% 
    dplyr::select(-sum_neighbour)
  
  return(grid_next)
    
}

grid_p1 <- grid_tidy

for(i in 1:6){
  
  grid_p1 <- cycle_p1(grid_p1)
  
}

grid_p1[["on"]] %>% sum()

##### Part 2 #####

cycle_p2 <- function(grid){
  
  grid_next <- 
    expand_grid(x = min_max_diff_1(grid[["x"]]), 
                y = min_max_diff_1(grid[["y"]]), 
                z = min_max_diff_1(grid[["z"]]), 
                w = min_max_diff_1(grid[["w"]])) %>% 
    left_join(grid, 
              by = c("x", "y", "z", "w")) %>% 
    mutate(on = ifelse(is.na(on), FALSE, on), 
           sum_neighbour = NA_integer_)
  
  for(i in seq_len(nrow(grid_next))){
    
    cell <- grid_next[i, ]
    neighbours <- expand_grid(x = min_max_diff_1(cell[["x"]]), 
                              y = min_max_diff_1(cell[["y"]]), 
                              z = min_max_diff_1(cell[["z"]]), 
                              w = min_max_diff_1(cell[["w"]])) %>% 
      inner_join(grid_next, 
                 by = c("x", "y", "z", "w")) %>% 
      anti_join(cell, 
                by = c("x", "y", "z", "w"))
    
    grid_next[["sum_neighbour"]][i] <- neighbours[["on"]] %>% sum()
    
  }
  
  grid_next <- grid_next %>% 
    mutate(on = case_when(on & sum_neighbour %in% c(2, 3) ~ TRUE, 
                          !on & sum_neighbour == 3 ~ TRUE, 
                          TRUE ~ FALSE)) %>% 
    dplyr::select(-sum_neighbour)
  
  return(grid_next)
  
}

grid_p2 <- grid_tidy %>% 
  mutate(w = 0)

for(i in 1:6){
  
  print(i)
  
  grid_p2 <- cycle_p2(grid_p2)
  
}

grid_p2[["on"]] %>% sum()
