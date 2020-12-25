library(tidyverse)
library(stringr)

# Load data ---------------------------------------------------------------

tickets_raw <- read_lines(here::here("2020", "raw_data", "day_16.txt"))

# Main --------------------------------------------------------------------

##### Part 1 #####

rules_raw <- tickets_raw[1:which(tickets_raw == "your ticket:")]

rules_tidy <- tibble(rules = rules_raw[!rules_raw %in% c("", "your ticket:")]) %>% 
  separate(rules, into = c("type", "rules"), ": ") %>% 
  separate(rules, into = str_c("rules_", 1:2), " or ") %>% 
  separate(rules_1, into = str_c(c("start_", "end_"), 1)) %>% 
  separate(rules_2, into = str_c(c("start_", "end_"), 2)) %>% 
  mutate_at(vars(contains(c("start", "end"))), 
            as.numeric)
  
rules_tidy <- rules_tidy %>% 
  gather("start_1_2", "start", contains(c("start"))) %>% 
  mutate(end = ifelse(start_1_2 == "start_1", end_1, end_2)) %>% 
  dplyr::select(type, start, end)

nearby_raw <- tickets_raw[which(tickets_raw == "nearby tickets:"):length(tickets_raw)]
nearby_raw <- nearby_raw[nearby_raw != "nearby tickets:"]

nearby_tidy <- vector("list", length(nearby_raw))

for(i in seq_along(nearby_raw)){
  
  nearby_tidy[[i]] <- tibble(times = nearby_raw[i] %>% 
                               str_split(",") %>% 
                               unlist() %>% 
                               as.integer(), 
                             ticket = i)
  
}

nearby_tidy <- do.call(bind_rows, nearby_tidy)

nearby_tidy <- nearby_tidy %>% 
  mutate(valid = FALSE)

for(i in seq_len(nrow(nearby_tidy))){
  
  valid_curr <- any(nearby_tidy[["times"]][i] >= rules_tidy[["start"]] & 
                      nearby_tidy[["times"]][i] <= rules_tidy[["end"]])
  
  nearby_tidy[["valid"]][i] <- valid_curr
  
}

nearby_tidy %>% 
  filter(valid == FALSE) %>% 
  .[["times"]] %>% 
  sum()

##### Part 2 #####

nearby_valid <- nearby_tidy %>% 
  group_by(ticket) %>% 
  summarise(valid = all(valid)) %>% 
  filter(valid)

nearby_valid <- nearby_tidy %>% 
  filter(ticket %in% nearby_valid[["ticket"]])

nearby_valid <- nearby_valid %>% 
  group_by(ticket) %>% 
  mutate(index = row_number()) %>% 
  ungroup()

index_to_rule <- tibble(index = unique(nearby_valid[["index"]]), 
                        rule = NA_character_)
index_to_rule[["rule_poss"]] <- vector("list", nrow(index_to_rule))

# which rules are possible for each index to match?
for(i in seq_len(nrow(index_to_rule))){
  
  index_times <- nearby_valid %>% 
    filter(index == index_to_rule[["index"]][i]) %>% 
    .[["times"]]
  
  for(j in seq_along(unique(rules_tidy[["type"]]))){
    
    type_curr <- unique(rules_tidy[["type"]])[j]
    
    rules_curr <- rules_tidy %>% 
      filter(type == type_curr)
    
    valid_1 <- index_times >= rules_curr[["start"]][1] & 
      index_times <= rules_curr[["end"]][1]
    
    valid_2 <- index_times >= rules_curr[["start"]][2] & 
      index_times <= rules_curr[["end"]][2]
     
     if(all(valid_1 | valid_2)){
       
       index_to_rule[["rule_poss"]][[i]] <- c(index_to_rule[["rule_poss"]][[i]], 
                                              type_curr)
    
     }
    
  }
  
}

# narrow down the rules to one per index
while(any(is.na(index_to_rule[["rule"]]))){
  
  poss <- lapply(index_to_rule[["rule_poss"]], length) %>% 
    unlist()

  which_1_poss <- which(poss == 1)
  rule_to_add <- index_to_rule[["rule_poss"]][[which_1_poss]]
  
  index_to_rule[["rule"]][which_1_poss] <- rule_to_add
    
  index_to_rule[["rule_poss"]] <- index_to_rule[["rule_poss"]] %>% 
    lapply(FUN = function(x) x[x != rule_to_add])

}

your_ticket <- tickets_raw[which(str_detect(tickets_raw, "your ticket:")) + 1] %>% 
  str_split(",") %>% 
  unlist() %>% 
  as.integer()

departure_indices <- index_to_rule %>% 
  filter(str_detect(rule, "departure")) %>% 
  .[["index"]]

prod(your_ticket[departure_indices])
