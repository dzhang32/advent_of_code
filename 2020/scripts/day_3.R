library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

grid_raw <- read_delim(here::here("2020", "raw_data", "day_3.txt"), 
                   delim = " ", 
                   col_names = FALSE)

# Main --------------------------------------------------------------------

##### Part 1 #####

# convert the grid into a matrix of 1/0s
# 1 being tree positions
grid_tidy <- grid_raw %>% 
  mutate(X1 = X1 %>% 
           str_replace_all("\\.", "0") %>% 
           str_replace_all("\\#", "1")) 

# what's the horizontal size of the grid  
grid_ncols <- grid_tidy[["X1"]][1] %>% str_count(".")
  
grid_tidy <- grid_tidy %>% 
  separate(X1, str_c("X", 0:(grid_ncols)), sep = "") %>% 
  dplyr::select(-X0) %>% 
  mutate_all(as.integer) %>% 
  as.matrix()

# now we need to sum each coord/cell that is traversed
# y-coords are just row number
y_coords <- 1:nrow(grid_tidy)

# x-coords go up by 3 and since we need to hit bottom
x_coords <- seq(from = 1, by = 3, length.out = nrow(grid_tidy))

# if x > ncols it returns to the %% of 31
x_coords <- ifelse(x_coords > 31, x_coords %% 31, x_coords)

# also need to replace 0's with 31
x_coords <- ifelse(x_coords == 0, 31, x_coords)

trees <- 0

for(i in seq_along(y_coords)){
  
  trees <- trees + grid_tidy[[y_coords[i], x_coords[i]]]
  
}

##### Part 2 #####

count_trees <- function(grid, step_right, step_down){
  
  grid_ncol <- ncol(grid)
  grid_nrow <- nrow(grid)
  
  y_coords <- seq(1, grid_nrow, step_down)
  x_coords <- seq(from = 1, 
                  by = step_right, 
                  length.out = length(y_coords))
  
  x_coords <- ifelse(x_coords > grid_ncol, x_coords %% grid_ncol, x_coords)
  x_coords <- ifelse(x_coords == 0, grid_ncol, x_coords)
  
  trees <- 0
  
  for(i in seq_along(y_coords)){
    
    trees <- trees + grid[[y_coords[i], x_coords[i]]]
    
  }
  
  return(trees)
  
}

trajects <- tibble(right = c(1, 3, 5, 7, 1), 
                   down = c(1, 1, 1, 1, 2))

trees <- vector("numeric", nrow(trajects))

for(i in seq_len(nrow(trajects))){
  
  trees[i] <- count_trees(grid_tidy, 
                          trajects[["right"]][i], 
                          trajects[["down"]][i])
  
}

prod(trees)




           