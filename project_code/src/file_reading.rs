//! Module for reading the csv files for both the graph edges and node statistics
use std::collections::HashMap;
use std::collections::HashSet;
use csv::Reader;
use std::error::Error;
use crate::data_analysis::NodeStats;

/// Reads given csv file of edges (given path), returns HashMap of u32 keys and HashSet<u32> values \
/// The HashMap functions as a **undirected** graph for further use
pub fn csv_to_hashmap (path: &str) -> Result<HashMap<u32, HashSet<u32>>, Box<dyn Error>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new(); 
    let mut rdr = Reader::from_path(path)?; // Build reader from path given as an argument
    for result in rdr.records() {
     let record = result?;
     let start_node = record.get(0).unwrap().parse::<u32>()?; // Parse both columns per row as u32
     let end_node = record.get(1).unwrap().parse::<u32>()?;
     // Entry one way
     map.entry(start_node) // Get HashSet for the start_node
     .or_insert_with(|| HashSet::new()) // If key doesn't exist, insert new HashSet
     .insert(end_node); // Insert end_node into HashSet
     // Reverse entry, makes graph undirected
     map.entry(end_node)
     .or_insert_with(|| HashSet::new())
     .insert(start_node);
 
    }
 
    return Ok(map)
 }

/// Once cliques are found using Bron_Kerbosch on u32 values (computationally faster),
/// remake the cliques using NodeStats objects for further data analysis \
///  ### Example
/// **Input:** vector of vectors containing u32s such as \[1, 2, 3\] \
/// The target.csv file is loaded into a vector with Serde deserialization \
/// For each node_id in the input vectors, this vector is referenced to find the matching NodeStruct \
/// When matching node is found (new_id == number), add deserialized NodeStats to new vector\
/// **Output:** vector of vector containing NodeStat objects such as \[NodeStat1, NodeStats2, NodeStats3\] \
/// 
/// **Note**  
/// Code has large computational complexity due to nested loops, a large number of cliques may take significant time \
/// Current code uses a min_value of 10 to get 14 cliques, for lower minimum values, this code may need to be optimized further \
/// **Possible optimization (ran out of time):** collecting ids from target csv as a Vec<u32> with a reader, 
/// then using that vec to find the row index for each clique node, and calling reader directly to that row. \
/// Requires loading the bit offset to have the reader find specific rows, too technical at the moment
pub fn load_target_file_replace_u32_cliques(path: &str, cliques: Vec<Vec<u32>>) -> Result<Vec<Vec<NodeStats>>, csv::Error> {
    let mut loaded_file: Vec<NodeStats> = Vec::new();
    let mut index_rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    for result in index_rdr.deserialize::<NodeStats>() {
        match result {
            Ok(record) => {
                loaded_file.push(record)
            }
            Err(err) => eprintln!("Error deserializing csv: {}", err)
        }
    }
    // Continue rework here
    let mut node_cliques: Vec<Vec<NodeStats>> = Vec::new(); 
    for clique in cliques {
        let mut node_clique: Vec<NodeStats> = Vec::new();
        for node_id in clique { 
            if let Some(matching_node) = loaded_file.iter().find(|node| node.new_id == node_id) {
                node_clique.push(matching_node.clone())
            }            
        }
        node_cliques.push(node_clique) // Pushes a Vec<NodeStats> onto another vector
    }
    Ok(node_cliques)
}
