//! Module for reading the csv file into a Hashmap that functions as an undirected graph
use std::collections::HashMap;
use std::collections::HashSet;
use csv::Reader;
use std::error::Error;


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


