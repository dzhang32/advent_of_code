library(tidyverse)
library(stringr)
library(tidygraph)
library(ggraph)

# Load data ---------------------------------------------------------------

rules_raw <- read_delim(here::here("2020", "raw_data", "day_7.txt"), 
                        delim = "\t", 
                        col_names = "rule")

# Main --------------------------------------------------------------------

##### Part 1 #####

# let's tidy up the bag rules 
# so that we have the separated bags and contents
# in a long format
rules_tidy <- rules_raw %>% 
  separate(rule, c("bag", "contents"), " bags contain ") %>% 
  separate(contents, str_c("contents_", 1:4), ", ") %>% 
  gather("dummy", "contents", contains("contents")) %>% 
  filter(!is.na(contents)) %>% 
  dplyr::select(-dummy) %>% 
  arrange(bag)

# split contents into number and type 
rules_tidy <- rules_tidy %>% 
  mutate(contents_num = contents %>% str_extract("^.."), 
         contents_type = contents %>% 
           str_remove("^..") %>% 
           str_remove(" bag.*"), 
         contents_num = ifelse(contents_num == "no", "0", contents_num), 
         contents_type = ifelse(contents_type == " other ", "none", contents_type),
         contents_num = contents_num %>% as.integer())

# create a graph of bags
# nodes are each of the bags
nodes <- rules_tidy %>% 
  dplyr::select(bag) %>% 
  filter(!duplicated(bag)) %>% 
  rowid_to_column("id")
  
# create edges express which bags are contained in which
# removing bags with no contents as these have no edges
edges <- rules_tidy %>% 
  filter(contents_num != 0) %>% 
  dplyr::select(bag, 
                contents_type, 
                contents_num)

# check we have id for all bags 
stopifnot(all(edges[["contents_type"]] %in% nodes[["bag"]]))

# convert edges to from/to/weight with ids
edges <- edges %>% 
  left_join(nodes, by = c("bag")) %>% 
  rename(from = id) %>% 
  left_join(nodes, by = c("contents_type" = "bag")) %>% 
  rename(to = id, weight = contents_num) 
  
bag_graph <- tbl_graph(nodes, edges, directed = TRUE) 

# given the directionality of the graph - container bags -> contents bag
# the number of bags that can contain shiny gold is the total 
# upstream nodes to the shiny gold - let's find this
shiny_gold <- nodes %>% 
  filter(bag == "shiny gold")

upstream <- bag_graph %>% 
  mutate(upstream = bfs_before(root = shiny_gold[["id"]], mode = "in"), 
         upstream = ifelse(is.na(upstream), FALSE, TRUE)) %>% 
  as_tibble()

sum(upstream[["upstream"]])

##### Part 2 #####

# convert nomenclature of bags into parent and child for 
# easier recognition 
rules_tidy <- rules_tidy %>% 
  dplyr::select(parent = bag, 
                child = contents_type, 
                num_child = contents_num) %>% 
  mutate(child = ifelse(child == " other", parent, child)) 

# for each node, the number of bags that node adds is equal 
# to the product of the weights of upstream paths
# let's manually cycle through each downstream bag and calc that number
# surely possible via graph theory, but can't figure it out
get_num_downstream <- function(root, rules){
  
  bag_curr_level <- rules_tidy %>% 
    filter(parent == root) %>% 
    mutate(path = num_child %>% as.list())
  
  num_total <- 0
  
  while(sum(bag_curr_level[["num_child"]]) != 0){
    
    # get total number of bags at this level 
    # and add it to the total
    # when the num_childs is 0 (no downstream bags)
    # prod will = 0, so will appropriately not add 
    num_bags_curr_level <- bag_curr_level[["path"]] %>% 
      lapply(prod) %>% 
      unlist() %>% 
      sum()
    
    num_total <- num_total + num_bags_curr_level
    
    # find the rules for bags down one level
    bag_next_level <- rules %>% 
      filter(parent %in% bag_curr_level[["child"]])
    
    bag_curr_level <- update_bag_level(bag_curr_level, 
                                       bag_next_level)
    
  }
  
  return(num_total)
  
}

update_bag_level <- function(bag_curr_level, bag_next_level){
  
  bag_curr_level <- bag_curr_level %>% 
    left_join(bag_next_level, 
              by = c("child" = "parent"), 
              suffix = c("_curr", "_next")) 
  
  # child of last level becomes parent
  # and next level child becomes new child
  bag_curr_level <- bag_curr_level %>% 
    dplyr::select(parent = child, 
                  child = child_next, 
                  num_child = num_child_next, 
                  path)
  
  # update paths with the edge weight
  # edge weight being the num of child bags
  for(i in seq_len(nrow(bag_curr_level))){
    
    bag_curr_level[["path"]][[i]] <- 
      c(bag_curr_level[["path"]][[i]], 
        bag_curr_level[["num_child"]][i])
    
  }
  
  return(bag_curr_level)
  
}

get_num_downstream(root = "shiny gold", 
                   rules = rules_tidy)

# # filter down to simple example to allow
# # visualisation for debugging and testing
# bag_to_vis <- "dull aqua"
# 
# # get id of bag to visualise the downstream of
# bag_to_vis <- nodes %>% 
#   filter(bag == bag_to_vis)
# 
# downstream <- bag_graph %>% 
#   mutate(downstream = bfs_before(root = bag_to_vis[["id"]], mode = "out")) %>% 
#   as_tibble() %>% 
#   filter(!is.na(downstream)) 
# 
# # keep only downstream nodes of and the bag itself
# bag_graph <- bag_graph %>%
#   activate(nodes) %>%
#   filter(bag %in% c(downstream[["bag"]], bag_to_vis[["bag"]]))
# 
# ggraph(bag_graph, layout = 'kk') +
#   geom_node_point() +
#   geom_edge_link2(aes(label = weight),
#                   alpha = 0.5,
#                   arrow = grid::arrow(length = unit(0.5, "cm"))) +
#   geom_node_text(aes(label = bag)) +
#   scale_edge_width(range = c(0.1, 1))
