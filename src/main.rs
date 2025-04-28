use std::collections::{HashMap, HashSet};

mod file_reading;
mod copied_alg;

fn bron_kerbosch(
    r:&HashSet<u32>,
    p: &mut HashSet<u32>,
    x: &mut HashSet<u32>,
    graph: &HashMap<u32, HashSet<u32>>,
    cliques: &mut Vec<Vec<u32>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 4 { //Only save cliques of 5 or greater to reduce output to relevant options
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

        bron_kerbosch(&new_r, &mut new_p, &mut new_x, graph, cliques);
        p.remove(&node);
        x.insert(node);
        }
    }


/// Found a definitive Bron-Kerbosch algorith online, checking if the two version give the same result
/// Definitive algorithm can be found at https://rosettacode.org/wiki/Bron%E2%80%93Kerbosch_algorithm#Rust
/// Code in "copied_alg" module
#[test]
fn test_alg() {    
    use file_reading;
    use copied_alg;

    let graph_result = file_reading::csv_to_hashmap("twitch_data/ENGB/musae_ENGB_edges.csv");
    let graph = match graph_result {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };
    
    let r1: HashSet<u32> = HashSet::new();
    let mut p1: HashSet<u32> = graph.keys().cloned().collect();
    let mut x1: HashSet<u32> = HashSet::new();

    let r2: HashSet<u32> = HashSet::new();
    let mut p2: HashSet<u32> = graph.keys().cloned().collect();
    let mut x2: HashSet<u32> = HashSet::new();
    
    let mut my_cliques: Vec<Vec<u32>> = Vec::new();
    let mut checking_cliques: Vec<Vec<u32>> = Vec::new();

    bron_kerbosch(&r1, &mut p1, &mut x1, &graph, &mut my_cliques);
    copied_alg::bron_kerbosch_v2(&r2, &mut p2, &mut x2, &graph, &mut checking_cliques);

    assert_eq!(my_cliques.sort(), checking_cliques.sort())

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = file_reading::csv_to_hashmap("twitch_data/ENGB/musae_ENGB_edges.csv")?;

    let r: HashSet<u32> = HashSet::new();
    let mut p: HashSet<u32> = graph.keys().cloned().collect();
    let mut x: HashSet<u32> = HashSet::new();

    let mut cliques: Vec<Vec<u32>> = Vec::new();
    bron_kerbosch(&r, &mut p, &mut x, &graph, &mut cliques);

    // Sort the cliques for consistent output
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort();

    // Print each clique
    for clique in sorted_cliques {
        println!("{:?}", clique);
    }

    Ok(())
}

