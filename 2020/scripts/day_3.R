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


           