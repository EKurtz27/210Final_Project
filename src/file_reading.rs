use std::collections::HashMap;
use std::collections::HashSet;
use csv::Reader;
use std::error::Error;

pub fn csv_to_hashmap (path: &str) -> Result<HashMap<u32, HashSet<u32>>, Box<dyn Error>> {
   let mut map: HashMap<u32, HashSet<u32>> = HashMap::new(); 
   let mut rdr = Reader::from_path(path)?;
   for result in rdr.records() {
    let record = result?;
    let start_node = record.get(0).unwrap().parse::<u32>()?;
    let end_node = record.get(1).unwrap().parse::<u32>()?;
    map.entry(start_node)
    .or_insert_with(|| HashSet::new())
    .insert(end_node);
   }

   return Ok(map)
}
