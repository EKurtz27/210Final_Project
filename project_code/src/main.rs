use std::collections::HashSet;
use std::io;
mod file_reading;
mod copied_alg;
mod data_analysis;
mod bron_kerbosch;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let edge_file_options = vec![
        "../twitch_data/DE/musae_DE_edges.csv",
        "../twitch_data/ENGB/musae_ENGB_edges.csv",
        "../twitch_data/ES/musae_ES_edges.csv",
        "../twitch_data/FR/musae_FR_edges.csv",
        "../twitch_data/PTBR/musae_PTBR_edges.csv",
        "../twitch_data/RU/musae_RU_edges.csv"
    ];
    
    let target_file_options = vec![
        "../twitch_data/DE/musae_DE_target.csv",
        "../twitch_data/ENGB/musae_ENGB_target.csv",
        "../twitch_data/ES/musae_ES_target.csv",
        "../twitch_data/FR/musae_FR_target.csv",
        "../twitch_data/PTBR/musae_PTBR_target.csv",
        "../twitch_data/RU/musae_RU_target.csv"
    ];
    // Input for setting region/language
    let mut lang_input = String::new();
    println!("Input a number for the language you'd like to see data analyzed for:");
    println!("0: German, 1: British English, 2: Spanish, 3: French, 4: Brazilian Portuguese, 5: Russian"); // Prompts user for input
    io::stdin().read_line(&mut lang_input).expect("Failure to read input");
    let region_choice  = lang_input.trim().parse::<usize>().expect("Please select from the numbers provided"); // Sets region/language

    // Input for setting minimum value
    let mut min_input = String::new();
    println!("Please enter the minimum size you'd like each saved clique to be:"); // Prompts user for input
    println!("Note that lower numbers equals more computation time and more image files created");
    io::stdin().read_line(&mut min_input).expect("Failure to read input");
    let min_value  = min_input.trim().parse::<u32>().expect("Please select from the numbers provided"); // Sets minimum threshold


    let graph = file_reading::csv_to_hashmap(edge_file_options[region_choice])?; // Creates the undirected graph
    // Initialize the inputs for the Bron-Kerbosch Algorithm
    let r: HashSet<u32> = HashSet::new();
    let mut p: HashSet<u32> = graph.keys().cloned().collect();
    let mut x: HashSet<u32> = HashSet::new();

    let mut cliques: Vec<Vec<u32>> = Vec::new();

    bron_kerbosch::run_bron_kerbosch(&r, &mut p, &mut x, &graph, &mut cliques, min_value); // Runs the Bron-Kerbosch algorithm to find cliques of at least 10

    // Sort the cliques for consistent output
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort();

    println!("Found {} cliques of at least size {}", sorted_cliques.len(), min_value);
    println!("This will create {} image files of at most 16 charts each", (sorted_cliques.len() as f32 / 16.0).ceil());
    println!("Would you like to continue? (y/n)");
    let mut continue_input = String::new();
    io::stdin().read_line(&mut continue_input).expect("Failure to read input");
    let keep_going  = continue_input.trim().to_string(); // confirms if the user wants to continue

    if keep_going == "y".to_string() {   

        let node_cliques = file_reading::load_target_file_replace_u32_cliques( // Replaces the u32 cliques with NodeStats cliques 
            target_file_options[region_choice], sorted_cliques).unwrap(); // Possible branching from here for more analysis

        let viewership_dists = data_analysis::viewership_distribution(&node_cliques); // Finds the viewership distributions for each clique

        data_analysis::plot_viewership_distributions(viewership_dists); // Generates the viewership_distributions.png file
    
    }
    else {
        println!("'y' was not selected, analysis will not progress. Please rerun the project to try again.")
    }
    Ok(())
}
