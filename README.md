# 210 Final_Project
Final data analysis project for DS210. Uses the Twitch Social Networks dataset from the Stanford SNAP database to compute relevant statistics about large creator groups. The project generates relevant statistics and visualizations to help understand whether identified maximal cliques (used to represent Twitch creator groups) generate positive viewership diffusion for all members or a concentration of viewership among a few streamers within the group. The project handles a variety of edge cases by utilizing user confirmations and batching of visualizations. The project uses an implementation of the Bron-Kerbosch Algorithm in order to identify maximal cliques.

## Running main.rs
The file first prompts for which dataset a user wishes to analyze, based on the available datasets, as well as asking the user for the minimum size each clique should be to be saved.  
After identifying the number of cliques, based on the given parameters, users will confirm they want to proceed to image generation.  
The file generates a plot of the distribution of viewership for each identified clique. Cliques are found through an implementation of the Bron-Kerbosch Algorithm.  
These distributions are graphed into bar charts and generated as files under the name "viewership_distributions.png".

## Structure for Further Research
More research can be done on each clique, as the cliques are converted to custom objects (NodeStats struct). These structs contain fields for the node (streamer) ID, number of views, internal Twitch rating of whether the streamer is "mature", and if the streamer is a partner. Working with the generated Vec<Vec<NodeStats>> can generate further statistical information.
