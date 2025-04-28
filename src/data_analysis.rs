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