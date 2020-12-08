library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

answers_raw <- read_lines(here::here("2020", "raw_data", "day_6.txt"))

# Main --------------------------------------------------------------------

##### Part 1 #####

answers_tidy <- tibble(answer = answers_raw)

# let's wrap this up into a function as seems to be a common 
# theme - groups differentiated by an empty line
group_by_empty_line <- function(df, info_colname){
  
  df <- df %>% 
    mutate(group = NA_integer_)
  
  # mark groups/individuals
  group <- 1L
  
  for(i in seq_len(nrow(df))){
    
    df[["group"]][i] <- group
    
    if(df[[info_colname]][i] == ""){
      
      group <- group + 1
      
    }
    
  }
  
  return(df)
  
}

answers_tidy <- group_by_empty_line(answers_tidy, "answer")

# concat all the answers of a group
answers_tidy <- answers_tidy %>% 
  group_by(group) %>% 
  summarise(answer = answer %>% str_c(collapse = ""))

# let's make a function to count the number of unique characters
# in each string
count_uniq_chr <- function(x){
  
  x <- x %>% 
    str_split("") %>% 
    lapply(function(x) length(unique(x))) %>% 
    unlist()
    
  return(x)
  
}

answers_p1 <- answers_tidy %>% 
  mutate(uniq_chr = count_uniq_chr(answer))

sum(answers_p1[["uniq_chr"]])

##### Part 2 #####

# let's recreate the grouped df
# but in this case remove the empty lines 
answers_tidy <- tibble(answer = answers_raw) %>% 
  group_by_empty_line("answer") %>% 
  filter(answer != "") 

# make a function to count the chr
# present in all individuals in a group
# x here will be a character vector with length the N of a group 
# and each element an answer from an individual
count_all_chr <- function(x){
  
  x <- str_split(x, "")
  
  all_chr <- x[[1]]
  
  for(i in seq_along(x)){
    
    all_chr <- all_chr[all_chr %in% x[[i]]]
    
  }
  
  all_chr <- length(all_chr)
  
  return(all_chr)
  
}

answers_p2 <- answers_tidy %>% 
  group_by(group) %>% 
  summarise(all_chr = count_all_chr(answer))

sum(answers_p2[["all_chr"]])

