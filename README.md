# 210 Final_Project
Final data analysis project for DS210. Uses the Twitch Social Networks dataset from the Stanford SNAP database to compute relevant statistics about large creator groups.

## Running main.rs
The file generates a plot of the distribution of viewership for each identified clique. Cliques are found through an implementation of the Bron-Kerbosch Algorithm. The minimum number of nodes needed to count each clique is hardcoded under a "min_value" variable. Smaller minimum values will result in increased computational time. These distributions are graphed into bar charts and generated as "viewership_distributions.png".

## Structure for Further Research
More research can be done on each clique, as the cliques are converted to custom objects (NodeStats struct). These structs contain fields for the node (streamer) id, number of views, internal Twitch rating of if the streamer is "mature", and if the streamer is a partner. Working with the generated Vec<Vec<NodeStats>> can generate further statistical information.
