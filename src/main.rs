use std::collections::{HashMap, HashSet};

mod file_reading;

fn bron_kerbosch(
    r:&HashSet<u32>,
    p: &mut HashSet<u32>,
    x: &mut HashSet<u32>,
    graph: &HashMap<u32, HashSet<u32>>,
    cliques: &mut Vec<Vec<u32>>,
    most_connected_node: &u32
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 4 { //Only save cliques of 5 or greater to reduce output to relevant options
            let mut clique: Vec<u32> = r.clone().into_iter().collect();
            clique.sort(); //Sort for readability
            cliques.push(clique);
        } 
    }
    return;

    let pivot = *most_connected_node;

    let neighbors = graph.get(&pivot).cloned().unwrap_or_default();
    let candidates: Vec<u32> = p.difference(&neighbors).cloned().collect();

    for node in candidates {
        let mut new_r = r.clone();
        new_r.insert(node.clone());

        let neighbors_of_node = graph.get(&node).cloned().unwrap_or_default();
        let mut new_p = p.intersection(&neighbors_of_node).cloned().collect::<HashSet<u32>>();

        let mut new_x = x.intersection(&neighbors_of_node).cloned().collect::<HashSet<u32>>();

        bron_kerbosch(&new_r, &mut new_p, &mut new_x, graph, cliques, most_connected_node); ///NEED TO CHANGE CONNECTED NODE SO IT WORKS FOR EACH CALL
    }
}



fn main() {
    let graph = file_reading::csv_to_hashmap("twitch_data/ENGB/musae_ENGB_edges.csv")?;
    for (key, values) in &graph {
        println!("Start Node: {}, Connections: {:?}", key, values);
    }
    let most_connected_node = (file_reading::find_connected_node(&graph)).unwrap();
    println!("Most connected node: {:?}", most_connected_node);
    println!("Connections to most connected node: {:?}", (&graph.get(&most_connected_node)).unwrap());

    let r: HashSet<u32> = HashSet::new();
    let r: HashSet<u32> = graph.keys().cloned().collect();
    let x: HashSet<u32> = HashSet::new();

    let mut cliques: Vec<Vec<u32>> = Vec::new();
}
