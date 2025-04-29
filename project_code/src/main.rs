use std::collections::HashSet;

mod file_reading;
mod copied_alg;
mod data_analysis;
mod bron_kerbosch;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = file_reading::csv_to_hashmap("twitch_data/ENGB/musae_ENGB_edges.csv")?; // Creates the undirected graph
    // Initialize the inputs for the Bron-Kerbosch Algorithm
    let r: HashSet<u32> = HashSet::new();
    let mut p: HashSet<u32> = graph.keys().cloned().collect();
    let mut x: HashSet<u32> = HashSet::new();

    let mut cliques: Vec<Vec<u32>> = Vec::new();
    let min_value: u32 = 10;
    bron_kerbosch::run_bron_kerbosch(&r, &mut p, &mut x, &graph, &mut cliques, min_value); // Runs the Bron-Kerbosch algorithm to find cliques of at least 10

    // Sort the cliques for consistent output
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort();


    let node_cliques = data_analysis::u32_cliques_to_node_cliques( // Replaces the u32 cliques with NodeStats cliques 
        "twitch_data/ENGB/musae_ENGB_target.csv", sorted_cliques).unwrap(); // Possible branching from here for more analysis

    let viewership_dists = data_analysis::viewership_distribution(&node_cliques); // Finds the viewership distributions for each clique

    data_analysis::plot_viewership_distributions(viewership_dists); // Generates the viewership_distributions.png file

    Ok(())
}

