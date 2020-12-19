library(tidyverse)
library(stringr)
library(compositions)

# Load data ---------------------------------------------------------------

program_raw <- read_delim(here::here("2020", "raw_data", "day_14.txt"), 
                          delim = "\t", 
                          col_names = "code")

# Main --------------------------------------------------------------------

##### Part 1 ####

program_tidy <- program_raw %>% 
  separate(code, c("type", "value"), " = ") %>% 
  separate(type, c("type", "address")) %>% 
  mutate(address = address %>% as.integer())

# as address is overwritten, we can keep only the final address
program_p1 <- program_tidy %>% 
  group_by(address) %>% 
  filter(is.na(address) | row_number() == max(row_number())) %>% 
  ungroup()

add_mask <- function(int, mask){
  
  int <- int %>% as.integer()
  int_bit36 <- compositions::binary(int, mb = 35)
  
  # find non-X positions
  mask_non_X_pos <- mask %>% 
    str_locate_all("[0-9]") %>% 
    .[[1]]
  
  for(j in seq_len(nrow(mask_non_X_pos))){
    
    mask_bit <- str_sub(mask, mask_non_X_pos[j], mask_non_X_pos[j])
    int_bit <- str_sub(int_bit36, mask_non_X_pos[j], mask_non_X_pos[j])
    
    if(mask_bit != int_bit){
      
      str_sub(int_bit36, mask_non_X_pos[j], mask_non_X_pos[j]) <- mask_bit
      
    }
    
  }
  
  return(int_bit36)
  
}
  
addresses <- vector("character", 
                    program_tidy[["address"]] %>% 
                      max(na.rm = TRUE))

for(i in seq_len(nrow(program_p1))){
  
  if(program_p1[["type"]][i] == "mask"){
    
    mask <- program_p1[["value"]][i]
    
  }else{
    
    int_bit36 <- add_mask(int = program_p1[["value"]][i], 
                          mask = mask)
    
    addresses[[program_p1[["address"]][i]]] <- int_bit36
  
  }
  
}

compositions::binary(42, mb = 35)
addresses <- addresses[addresses != ""]

addresses %>% 
  compositions::unbinary() %>% 
  gmp::as.bigz() %>% 
  sum()

##### Part 2 ####

# first, let's convert the convert the memory addresses 
# using the new masking rule
convert_address <- function(mask, address, value){
  
  int_bit36 <- address %>% 
    as.integer() %>% 
    compositions::binary(mb = 35)
  
  addresses <- add_mask_p2(int_bit36, mask)
  
  # create all possible addresses using expand
  addresses <- 
    tibble(addresses = c(addresses %>% str_replace_all("X", "0"),
                         addresses %>% str_replace_all("X", "1"))) %>% 
    separate(addresses, into = str_c("X", 0:36), "") %>% 
    select(-X0) 
  
  addresses <- addresses %>% 
    expand(!!! syms(colnames(addresses))) %>% 
    apply(MARGIN = 1, 
          FUN = str_c, 
          collapse = "") %>% 
    compositions::unbinary() %>% 
    as.character()

  addresses <- tibble(address = addresses,
                      value = value)
  
  return(addresses)
    
}

add_mask_p2 <- function(int_bit36, mask){
  
  for(j in seq_len(36)){
    
    mask_bit <- str_sub(mask, j, j)
    int_bit <- str_sub(int_bit36, j, j)
    
    if(mask_bit == "0"){
      
      next
      
    }else{
      
      str_sub(int_bit36, j, j) <- mask_bit
      
    }
    
  }
  
  return(int_bit36)
  
}

address_val_all <- list()

for(i in seq_len(nrow(program_tidy))){
  
  print(i)
  
  if(program_tidy[["type"]][i] == "mask"){
    
    mask <- program_tidy[["value"]][i]
    
  }else{
    
    address_val_curr <- convert_address(mask = mask, 
                                        address = program_tidy[["address"]][i], 
                                        value = program_tidy[["value"]][i])
    
    address_val_all[[i]] <- address_val_curr
    
  }
  
}

address_val_all <- do.call(bind_rows, address_val_all)

# we can filter again out the addresses that will be overwritten
address_val_all <- address_val_all %>% 
  group_by(address) %>% 
  filter(!duplicated(address, fromLast = TRUE)) %>% 
  ungroup() %>% 
  mutate(value = value %>% as.double())

address_val_all[["value"]] %>% 
  gmp::as.bigz() %>% 
  sum()

