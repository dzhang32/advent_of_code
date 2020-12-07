library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

passports_raw <- readr::read_lines(here::here("2020", "raw_data", "day_4.txt"))

# Main --------------------------------------------------------------------

##### Part 1 #####

# we need to wrangle this 
# ideally to have one person per row
passports_tidy <- tibble(info = passports_raw, 
                         ind = NA_integer_) 

# mark groups/individuals
ind <- 1L

for(i in seq_len(nrow(passports_tidy))){
  
  passports_tidy[["ind"]][i] <- ind
  
  if(passports_tidy[["info"]][i] == ""){
    
    ind <- ind + 1
    
  }
  
}

passports_tidy <- passports_tidy %>% 
  group_by(ind) %>% 
  summarise(info = str_c(info, collapse = " "))

# now let's check whether we have each of 7 criteria needed 
needed <- c("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid")

for(i in seq_along(needed)){
  
  passports_tidy <- passports_tidy %>% 
    mutate(!!needed[i] := str_detect(info, needed[i]))
  
}

passports_valid <- passports_tidy %>% 
  mutate(valid = (byr + iyr + eyr + hgt + hcl + ecl + pid) == 7) %>% 
  filter(valid) 

passports_valid %>% 
  nrow()

##### Part 2 #####

# now we have to tidy our data so that 
# each metric sits in it's own column
passports_tidy <- passports_valid %>% 
  dplyr::select(ind, info)

# first sort all the columns so their in the same order
# remove cid and blank entries
info_tidy <- 
  passports_tidy[["info"]] %>% 
  str_split(pattern = " ") %>% 
  lapply(sort) %>% 
  lapply(function(x) x[!str_detect(x, "cid")]) %>% 
  lapply(function(x) x[x != ""]) 

# check they're all the same length
stopifnot(all(unlist(lapply(info_tidy, length)) == 7))

passports_tidy <- passports_tidy %>% 
  mutate(info = lapply(info_tidy, str_c, collapse = " ") %>% unlist()) %>% 
  separate(info, 
           into = sort(needed),
           sep = " ")


tidy_info <- function(x){
  
  x %>% 
    str_replace(".*", "") %>% 
    as.
  
}

# now filter by the criteria 
passports_valid <- passports_tidy %>% 
  mutate_if(.predicate = is.character, 
            str_replace, 
            pattern = ".*:", 
            replacement = "") %>%
  mutate(byr = byr %>% as.integer(), 
         iyr = iyr %>% as.integer(), 
         eyr = eyr %>% as.integer(), 
         hgt_unit = hgt %>% str_extract("..$"), 
         hgt_num = hgt %>% 
           str_replace("..$", "") %>% 
           as.integer(), 
         hcl_hash = hcl %>% str_extract("^."), 
         hcl_value = hcl %>% str_replace("^.", ""),
         pid_int = pid %>% 
           as.integer()) %>% 
  filter(str_count(byr) == 4, byr >= 1920, byr <= 2002, 
         str_count(iyr) == 4, iyr >= 2010, iyr <= 2020, 
         str_count(eyr) == 4, eyr >= 2020, eyr <= 2030, 
         hgt_unit %in% c("cm", "in"), 
         (hgt_unit == "cm" & hgt_num >= 150) | (hgt_unit == "in" & hgt_num >= 59), 
         (hgt_unit == "cm" & hgt_num <= 193) | (hgt_unit == "in" & hgt_num <= 76), 
         hcl_hash == "#", 
         str_count(hcl_value) == 6,
         ecl %in% c("amb", "blu", "brn", "gry", "grn", "hzl", "oth"), 
         !is.na(pid_int), 
         str_count(pid) == 9) 

nrow(passports_valid)