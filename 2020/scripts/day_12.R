library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

directions_raw <- read_delim(here::here("2020", "raw_data", "day_12.txt"), 
                             delim = "\t", 
                             col_names = "instruction")

# Main --------------------------------------------------------------------

##### Part 1 #####

directions_tidy <- directions_raw %>% 
  mutate(direction = instruction %>% 
           str_extract("^."), 
         value = instruction %>% 
           str_remove("^.") %>% 
           as.integer())

update_facing <- function(facing, direction){
  
  compass <- c("N", "E", "S", "W")
  
  if(direction[["direction"]] %in% c("R", "L")){
    
    turn_by <- direction[["value"]] / 90
    
    if(direction[["direction"]] == "L"){
      
      turn_by <- -turn_by
      
    }
    
    facing <- which(compass %in% facing) + turn_by
    
    if(facing < 1){
      
      facing <- facing + 4
      
    }else if(facing > 4){
      
      facing <- facing - 4
      
    }
    
    facing <- compass[facing]
    
  }
  
  return(facing)
  
}

move_p1 <- function(facing, direction, position){

  if(direction[["direction"]] == "F"){
    
    position[[facing]] <- position[[facing]] + direction[["value"]]
    
  }else if(direction[["direction"]] %in% c("N", "E", "S", "W")){
    
    position[[direction[["direction"]]]] <- 
      position[[direction[["direction"]]]] + direction[["value"]]
    
  }
  
  return(position)
  
}

facing <- "E"
position <- c("N" = 0, 
              "E" = 0, 
              "S" = 0, 
              "W" = 0)

for(i in seq_len(nrow(directions_tidy))){
  
  direction_curr <- directions_tidy[i, ]
  
  facing <- update_facing(facing, direction_curr)
  
  position <- move_p1(facing, direction_curr, position)
  
}

NS_dis <- abs(position[["N"]] - position[["S"]])
EW_dis <- abs(position[["E"]] - position[["W"]])

NS_dis + EW_dis

##### Part 2 #####

update_waypoint <- function(way_point, direction){
  
  compass <- c("N", "E", "S", "W")
  
  if(direction[["direction"]] %in% c("R", "L")){
    
    way_point_prev <- way_point
    turn_by <- direction[["value"]] / 90
    
    if(direction[["direction"]] == "L"){
      
      turn_by <- -turn_by
      
    }
    
    for(j in seq_along(way_point_prev)){
      
      index_to_fill <- j + turn_by
      
      if(index_to_fill < 1){
        
        index_to_fill <- index_to_fill + 4
        
      }else if(index_to_fill > 4){
        
        index_to_fill <- index_to_fill - 4
        
      }
      
      way_point[index_to_fill] <- way_point_prev[[j]]
      
    }
    
  }else if(direction[["direction"]] %in% c("N", "E", "S", "W")){
    
    way_point[[direction[["direction"]]]] <- 
      way_point[[direction[["direction"]]]] + direction[["value"]]
    
  }
  
  return(way_point)
  
}

move_p2 <- function(way_point, direction, position){
  
  if(direction[["direction"]] == "F"){
    
    position <- position + way_point * direction[["value"]] 
    
  }
  
  return(position)
  
}

way_point <- c("N" = 1, 
               "E" = 10, 
               "S" = 0, 
               "W" = 0)

position <- c("N" = 0, 
              "E" = 0, 
              "S" = 0, 
              "W" = 0)

for(i in seq_len(nrow(directions_tidy))){
  
  direction_curr <- directions_tidy[i, ]
  
  way_point <- update_waypoint(way_point, direction_curr)
  
  position <- move_p2(way_point, direction_curr, position)
  
}

NS_dis <- abs(position[["N"]] - position[["S"]])
EW_dis <- abs(position[["E"]] - position[["W"]])

NS_dis + EW_dis

