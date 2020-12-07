library(tidyverse)

# Load data ---------------------------------------------------------------

raw_entries <- read_delim("raw_data/day_1.txt", delim = "\t", col_names = "entry")

# Main --------------------------------------------------------------------

##### Part 1 #####

# these entries can't have a pair that sums to 2020
entries_p1 <- raw_entries %>% 
  dplyr::filter(entry <= 2020 - min(entry), 
                entry >= 2020 - max(entry))

# find diff between entries and 2020
# then filter for only entries that match a diff
entries_p1 <- entries_p1 %>% 
  mutate(entry_diff_2020 = 2020 - entry) %>% 
  filter(entry %in% entry_diff_2020)

entries_p1[["entry"]][1] * entries_p1[["entry"]][2]

##### Part 2 #####

# use expand to create all possible combinations of entries
# inefficient but simple
# then find the difference between the two entries and 2020
# similar to above part 1
entries_p2 <- raw_entries %>% 
  mutate(entry_2 = entry) %>% 
  expand(entry, entry_2) %>% 
  mutate(entry_diff_2020 = (2020 - entry - entry_2)) %>% 
  filter(entry %in% entry_diff_2020, 
         !duplicated(entry))
  
entries_p2[["entry"]][1] * entries_p2[["entry"]][2] * entries_p2[["entry"]][3]

  
