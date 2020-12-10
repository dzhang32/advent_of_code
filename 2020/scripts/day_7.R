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

get_num_bags <- function(scores, node, path, ...){
  
  if(nrow(path) == 0){
    
    return(NA_integer_)
    
  }
  
  curr_node <- tibble(node = node,
                      parent = path[["node"]][nrow(path)])
  
  path <- path %>% 
    bind_rows(curr_node) %>% 
    filter(!is.na(parent)) %>% 
    left_join(.E(), 
              by = c("parent" = "from", 
                     "node" = "to"))
  
  num_bags <- prod(path[["weight"]]) %>% 
    as.integer()
  
  return(num_bags)
  
}

check <- bag_graph %>%
  mutate(num_bags = map_dfs_int(root = shiny_gold[["id"]], 
                                .f = get_num_bags, 
                                scores = scores, 
                                mode = "out")) %>% 
  as_tibble() %>% 
  filter(!is.na(num_bags))

sum(check[["num_bags"]])


# filter down to simple example to allow
# visualisation for debugging and testing purposes
# bag_graph <- bag_graph %>%
#   activate(edges) %>%
#   filter(bag == "shiny gold" | contents_type == "shiny gold" | contents_type == "drab silver")
# 
# edges <- bag_graph %>%
#   activate(edges) %>%
#   as_tibble()
# 
# bag_graph <- bag_graph %>%
#   activate(nodes) %>%
#   filter(bag %in% c(edges[["bag"]], edges[["contents_type"]])) %>%
#   mutate(id = row_number())
# 
# ggraph(bag_graph, layout = 'kk') +
#   geom_node_point() +
#   geom_edge_link2(aes(width = weight),
#                   alpha = 0.5,
#                   arrow = grid::arrow(length = unit(0.5, "cm"))) +
#   geom_node_text(aes(label = bag)) +
#   scale_edge_width(range = c(0.1, 1))
