use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeStats {
    pub new_id: u32,
    pub views: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mature: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub partner: bool,
}
fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where 
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(s, &["True", "False"])),
    }
}

pub fn u32_cliques_to_node_cliques(path: &str, cliques: Vec<Vec<u32>>) -> Result<Vec<Vec<NodeStats>>, csv::Error> {
    let mut node_cliques: Vec<Vec<NodeStats>> = Vec::new();
    for clique in cliques {
        let mut node_clique: Vec<NodeStats> = Vec::new();
        for node in clique {
            let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_path(path)?;
            for result in rdr.deserialize::<NodeStats>() {
                match result {
                    Ok(record) => {
                        if record.new_id == node {
                            node_clique.push(record)
                        }
                    }
                    Err(err) => eprintln!("Error deserializing csv: {}", err)
                }

            }
            
        }
        node_cliques.push(node_clique)
    }
    Ok(node_cliques)
}

pub fn viewership_distribution (cliques: &Vec<Vec<NodeStats>>) -> Vec<Vec<f32>> {
    let mut all_view_dists = Vec::new();
    for clique in cliques {
        let sum = clique.iter().fold(0, |acc, node| acc + node.views);
        let mut clique_viewership_dist = Vec::new();
        for node in clique {
            let node_views_percent: f32 = node.views as f32 / sum as f32;
            clique_viewership_dist.push(node_views_percent);
        }
        all_view_dists.push(clique_viewership_dist);
    }
    return all_view_dists
}
#[test]
fn test_distributions () {
    let node1 = NodeStats {
        new_id: 1,
        views: 100,
        mature: true,
        partner: false,
    };
    let node2 = NodeStats {
        new_id: 1,
        views: 200,
        mature: true,
        partner: false,
    };
    let node3 = NodeStats {
        new_id: 1,
        views: 300,
        mature: true,
        partner: false,
    };  
    let clique1: Vec<NodeStats> = vec![node1, node2, node3];
    let clique2: Vec<NodeStats> = vec![node1, node3];
    let cliques = vec![clique1, clique2];

    let dists = viewership_distribution(&cliques);
    let true_dists: Vec<Vec<f32>> = vec![vec![(100.0/600.0), (200.0/600.0), (300.0/600.0)], vec![(100.0/400.0), (300.0/400.0)]];

    assert_eq!(dists, true_dists, "Distributions not aligning");

}