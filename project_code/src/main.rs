use std::collections::{HashMap, HashSet};

mod file_reading;
mod copied_alg;
mod data_analysis;

fn bron_kerbosch(
    r:&HashSet<u32>,
    p: &mut HashSet<u32>,
    x: &mut HashSet<u32>,
    graph: &HashMap<u32, HashSet<u32>>,
    cliques: &mut Vec<Vec<u32>>,
    min_value: u32
) {
    if p.is_empty() && x.is_empty() {
        if r.len() >= min_value as usize { //Only save cliques of 5 or greater to reduce output to relevant options
            let mut clique: Vec<u32> = r.clone().into_iter().collect();
            clique.sort();
            cliques.push(clique);
        } 
        return;
    }
    
    let pivot = p
        .union(x)
        .max_by_key(|possible_node| graph.get(*possible_node).map(|neighbors| neighbors.len()))
        .unwrap();

    
    let neighbors = graph.get(&pivot).cloned().unwrap_or_default();
    let candidates: Vec<u32> = p.difference(&neighbors).cloned().collect();

    for node in candidates {
        let mut new_r = r.clone();
        new_r.insert(node.clone());

        let neighbors_of_node = graph.get(&node).cloned().unwrap_or_default();
        let mut new_p = p.intersection(&neighbors_of_node).cloned().collect::<HashSet<u32>>();

        let mut new_x = x.intersection(&neighbors_of_node).cloned().collect::<HashSet<u32>>();

        bron_kerbosch(&new_r, &mut new_p, &mut new_x, graph, cliques, min_value);
        p.remove(&node);
        x.insert(node);
        }
    }


/// Found a definitive Bron-Kerbosch algorith online, checking if the two version give the same result
/// Definitive algorithm can be found at https://rosettacode.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Rust
/// Code in "copied_alg" module
#[test]
fn test_alg_smallscale() {    
    use copied_alg;

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

    bron_kerbosch(&r1, &mut p1, &mut x1, &graph, &mut my_cliques, 2);
    copied_alg::bron_kerbosch_v2(&r2, &mut p2, &mut x2, &graph, &mut checking_cliques, 2);
    
    println!("my_cliques: {:?}", my_cliques);
    println!("checking_cliques: {:?}", checking_cliques);
    assert_eq!(my_cliques.sort(), checking_cliques.sort())

}


#[test]
fn test_alg_largescale() {
    use copied_alg;
    use file_reading;
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
    println!("Graph creation finished");
    let graph_start = graph.clone();

    let r1: HashSet<u32> = HashSet::new();
    let mut p1: HashSet<u32> = graph.keys().cloned().collect();
    let mut x1: HashSet<u32> = HashSet::new();

    let r2: HashSet<u32> = HashSet::new();
    let mut p2: HashSet<u32> = graph.keys().cloned().collect();
    let mut x2: HashSet<u32> = HashSet::new();

    let mut my_cliques: Vec<Vec<u32>> = Vec::new();
    let mut checking_cliques: Vec<Vec<u32>> = Vec::new();

    bron_kerbosch(&r1, &mut p1, &mut x1, &graph, &mut my_cliques, 5);
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = file_reading::csv_to_hashmap(
        "twitch_data/ENGB/musae_ENGB_edges.csv")?;

    let r: HashSet<u32> = HashSet::new();
    let mut p: HashSet<u32> = graph.keys().cloned().collect();
    let mut x: HashSet<u32> = HashSet::new();

    let mut cliques: Vec<Vec<u32>> = Vec::new();
    let min_value: u32 = 10;
    bron_kerbosch(&r, &mut p, &mut x, &graph, &mut cliques, min_value);

    // Sort the cliques for consistent output
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort();

    println!("Cliques: {}", &sorted_cliques.len());
    println!("Clique numbers: {:?}", &sorted_cliques[0]);

    let node_cliques = data_analysis::u32_cliques_to_node_cliques(
        "twitch_data/ENGB/musae_ENGB_target.csv", sorted_cliques).unwrap();

    let viewership_dists = data_analysis::viewership_distribution(&node_cliques);

    data_analysis::plot_viewership_distributions(viewership_dists);

    Ok(())
}

