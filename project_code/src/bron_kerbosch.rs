//! Module containing my implementation of the Bron-Kerbosch algorithm and associated tests, which references the copied_alg module

use std::collections::{HashMap, HashSet};
/// Runs the ['Bron-Kerbosch Algorithm'] \
/// ### Inputs
/// r: HashSet of nodes in the clique currently being built (initially empty) \
/// p: HashSet of possible nodes left to explore (initially all nodes) \
/// x: HashSet of excluded nodes, that have already been processed (initially empty) \
/// graph: HashMap of u32 keys and HashSet<u32> values, where the HashSet represents edges from the key to other nodes \
/// cliques: Vector of u32 vectors, initially empty. This is the output with all identified cliques \
/// min_value: a u32 value that sets the threshold for how many nodes must be in a clique to be saved in the cliques vector (inclusive) \
/// ### Algorithm Logic
/// If there are no unprocessed nodes (p is empty) and no processed nodes (x is empty) left { \
///     add r to cliques if r passes min_value threshold  \
/// } \
/// Choose a central node based on which remaining node has the most neighbors \
/// Define possible candidates based on this central (pivot) node \
/// 
/// Recursively call the algorithm on updated inputs: \
/// the central node is added to r \
/// p is now the unprocessed neighbors of the central node \
/// x is now the processed neighbors of the central node \
/// 
/// Add the central node to x and remove from p, since it has now been processed \
/// 
/// ### Output
/// Cliques is a vector of vectors, containing each of the cliques that passes the length threshold
/// 
/// ['Bron-Kerbosch Algorithm']: https://rosettacode.org/wiki/Bron%E2%80%93Kerbosch_algorithm
pub fn run_bron_kerbosch(
    r: &HashSet<u32>,
    p: &mut HashSet<u32>,
    x: &mut HashSet<u32>,
    graph: &HashMap<u32, HashSet<u32>>,
    cliques: &mut Vec<Vec<u32>>,
    min_value: u32
) {
    if p.is_empty() && x.is_empty() {
        if r.len() >= min_value as usize { //Only save cliques that pass set threshold to reduce output to relevant options
            let mut clique: Vec<u32> = r.clone().into_iter().collect();
            clique.sort();
            cliques.push(clique);
        } 
        return;
    }
    
    let pivot = p // Choose pivot based on remaining node with the most neighbors
        .union(x)
        .max_by_key(|possible_node| graph.get(*possible_node).map(|neighbors| neighbors.len())) // Counts each node's neighbors, selects the max
        .unwrap();

    
    let neighbors = graph.get(&pivot).cloned().unwrap_or_default();
    let candidates: Vec<u32> = p.difference(&neighbors).cloned().collect(); // Candidates are remaining unprocessed nodes that aren't neighbors to pivot (if we add a neighbor, we would also add pivot node)

    for node in candidates {
        let mut new_r = r.clone();
        new_r.insert(node.clone()); // r now includes candidate node

        let neighbors_of_node = graph.get(&node).cloned().unwrap_or_default(); 
        let mut new_p = p.intersection(&neighbors_of_node).cloned().collect::<HashSet<u32>>(); // New p equals all unprocessed neighbors

        let mut new_x = x.intersection(&neighbors_of_node).cloned().collect::<HashSet<u32>>(); // New x equals all processed neighbors

        run_bron_kerbosch(&new_r, &mut new_p, &mut new_x, graph, cliques, min_value); // Recursively call until p and x are empty (see start of algorithm)
        p.remove(&node); // the candidate node has now been processed and should move from p to x
        x.insert(node);
        }
    }


/// Found a definitive Bron-Kerbosch algorith online, checking if the two version give the same result
/// Definitive algorithm can be found at https://rosettacode.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Rust
/// Code in "copied_alg" module
#[test]
fn test_alg_smallscale() {    
    use std::collections::{HashMap, HashSet};
    use crate::copied_alg;

    let input: Vec<(u32, u32)> = vec![
        (1, 2),
        (2, 1),
        (1, 3),
        (3, 1),
        (2, 3),
        (3, 2),
        (4, 5),
        (5, 4),
        (4, 6),
        (6, 4),
        (5, 6),
        (6, 5),
    ];

    let mut graph: HashMap<u32, HashSet<u32>>  = HashMap::new();
    for (node, dest) in input.iter() {
        graph
            .entry(*node)
            .or_insert_with(HashSet::new)
            .insert(*dest);
    }

for (k, v) in graph.iter() {
    println!("k: {}, v: {:?}", k, v);
}

    let r1: HashSet<u32> = HashSet::new();
    let mut p1: HashSet<u32> = graph.keys().cloned().collect();
    let mut x1: HashSet<u32> = HashSet::new();

    let r2: HashSet<u32> = HashSet::new();
    let mut p2: HashSet<u32> = graph.keys().cloned().collect();
    let mut x2: HashSet<u32> = HashSet::new();
    
    let mut my_cliques: Vec<Vec<u32>> = Vec::new();
    let mut checking_cliques: Vec<Vec<u32>> = Vec::new();

    run_bron_kerbosch(&r1, &mut p1, &mut x1, &graph, &mut my_cliques, 2);
    copied_alg::bron_kerbosch_v2(&r2, &mut p2, &mut x2, &graph, &mut checking_cliques, 2);
    
    println!("my_cliques: {:?}", my_cliques);
    println!("checking_cliques: {:?}", checking_cliques);
    assert_eq!(my_cliques.sort(), checking_cliques.sort())

}


#[test]
fn test_alg_largescale() {
    use crate::copied_alg;
    use crate::file_reading;
    use std::collections::HashSet;

    // Handle the Result returned by csv_to_hashmap
    let graph_result = file_reading::csv_to_hashmap(
        "twitch_data/ENGB/musae_ENGB_edges.csv",
        );
    let graph = match graph_result {
        Ok(graph) => graph, // If successful, extract the graph
        Err(err) => {
            panic!("Error reading graph: {}", err); // Handle the error
        }
    };

    let graph_start = graph.clone();

    let r1: HashSet<u32> = HashSet::new();
    let mut p1: HashSet<u32> = graph.keys().cloned().collect();
    let mut x1: HashSet<u32> = HashSet::new();

    let r2: HashSet<u32> = HashSet::new();
    let mut p2: HashSet<u32> = graph.keys().cloned().collect();
    let mut x2: HashSet<u32> = HashSet::new();

    let mut my_cliques: Vec<Vec<u32>> = Vec::new();
    let mut checking_cliques: Vec<Vec<u32>> = Vec::new();

    run_bron_kerbosch(&r1, &mut p1, &mut x1, &graph, &mut my_cliques, 5);
    copied_alg::bron_kerbosch_v2(&r2, &mut p2, &mut x2, &graph, &mut checking_cliques, 5);

    println!("my_cliques: {:?}", &my_cliques.len());
    println!("checking_cliques: {:?}", &checking_cliques.len());

    let graph_end = graph.clone();

    // Sort and compare the cliques
    my_cliques.sort();
    checking_cliques.sort();


    assert_eq!(graph_start, graph_end, "Graph is mutated");
    assert_eq!(my_cliques, checking_cliques, "Lengths do not match");
}