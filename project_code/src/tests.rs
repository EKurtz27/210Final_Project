//! Module containing all tests for the project
use std::collections::HashSet;
use crate::{copied_alg, bron_kerbosch, data_analysis, file_reading};


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

/// Same concept as the small-scale test, however this tests on an actual data set using the csv_to_hashmap function
/// Uses the British English data with a minimum value of 5
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
/// Tests the viewership_distributions function against a ground truth
#[test]
fn test_distributions () {
    let node1 = NodeStats {
        new_id: 1,
        views: 100,
        mature: true,
        partner: false,
    };
    let node2 = NodeStats {
        new_id: 2,
        views: 200,
        mature: true,
        partner: false,
    };
    let node3 = NodeStats {
        new_id: 3,
        views: 300,
        mature: true,
        partner: false,
    };  
    let clique1: Vec<NodeStats> = vec![node1, node2, node3];
    let clique2: Vec<NodeStats> = vec![node1, node3];
    let cliques = vec![clique1, clique2];

    let dists = viewership_distribution(&cliques);
    let true_dists: Vec<Vec<(u32, f32)>> = vec![
        vec![(1, 100.0/600.0), (2, 200.0/600.0), (3, 300.0/600.0)], 
        vec![(1, 100.0/400.0), (3, 300.0/400.0)]
        ];

    assert_eq!(dists, true_dists, "Distributions not aligning");

}
