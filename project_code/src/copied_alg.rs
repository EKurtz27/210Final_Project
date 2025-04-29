use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
pub fn bron_kerbosch_v2(
    r: &HashSet<u32>,
    p: &mut HashSet<u32>,
    x: &mut HashSet<u32>,
    g: &HashMap<u32, HashSet<u32>>,
    cliques: &mut Vec<Vec<u32>>,
    min_value: usize   
) {
    if p.is_empty() && x.is_empty() {
        if r.len() >= min_value {
            let mut clique: Vec<u32> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
    .union(x)
    .max_by_key(|possible_node| {
        g
            .get(possible_node)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .filter(|n| n != possible_node) // Compare by full object equality
                    .count()
            })
            .unwrap_or(0)
    });


    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<u32> = p.difference(&neighbors).cloned().collect();

        // Sort the candidates to ensure the same order in recursion
        let mut sorted_candidates = candidates.clone();
        sorted_candidates.sort();

        for v in sorted_candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p.intersection(&neighbors_v).cloned().collect::<HashSet<u32>>();

            // New X is X ∩ N(v)
            let mut new_x = x.intersection(&neighbors_v).cloned().collect::<HashSet<u32>>();

            // Recursive call
            bron_kerbosch_v2(&new_r, &mut new_p, &mut new_x, g, cliques, min_value);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}