//! Module focused on data manipulation and visualization.
use serde::Deserialize;
/// Represents relevant statistics for each node, found in target.csv files
#[derive(Debug, Deserialize, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeStats {
    pub new_id: u32,
    pub views: u32,
    #[serde(deserialize_with = "deserialize_bool")] // Custom deserialize function to override capitalization
    pub mature: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub partner: bool,
}
/// Definition of custom deserialization \
/// Matches a capitalized statement with its bool value \
/// Lifted from StackOverflow answer ['here']
/// 
/// 
/// ['here']: https://stackoverflow.com/questions/70114905/how-to-deserialize-a-string-field-to-bool
fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where 
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "True" => Ok(true), // Match capitalized bool statements with Rust readable bools
        "False" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(s, &["True", "False"])), // Handles error of non-bool value
    }
}
/// Once cliques are found using Bron_Kerbosch on u32 values (computationally faster),
/// remake the cliques using NodeStats objects for further data analysis \
///  # Example
/// **Input:** vector of vectors containing u32s such as \[1, 2, 3\] \
/// For each number, read the target.csv file with Serde deserialization \
/// When matching node is found (new_id == number), add deserialized NodeStats to new vector\
/// **Output:** vector of vector containing NodeStat objects such as \[NodeStat1, NodeStats2, NodeStats3\] \
/// 
/// **Note**  
/// Code has large computational complexity due to nested loops, a large number of cliques may take significant time \
/// Current code uses a min_value of 10 to get 14 cliques, for lower minimum values, this code may need to be optimized further
pub fn u32_cliques_to_node_cliques(path: &str, cliques: Vec<Vec<u32>>) -> Result<Vec<Vec<NodeStats>>, csv::Error> {
    let mut node_cliques: Vec<Vec<NodeStats>> = Vec::new(); 
    for clique in cliques {
        let mut node_clique: Vec<NodeStats> = Vec::new();
        for node in clique { 
            let mut rdr = csv::ReaderBuilder::new() // Build reader from give path (target.csv file)
            .has_headers(true) // target.csv files have a header
            .from_path(path)?;
            for result in rdr.deserialize::<NodeStats>() {
                match result {
                    Ok(record) => {
                        if record.new_id == node {
                            node_clique.push(record) // Finds the record that matches each node, adds it to a vector to reconstruct Vec<Vec<>> format
                        }
                    }
                    Err(err) => eprintln!("Error deserializing csv: {}", err)
                }

            }
            
        }
        node_cliques.push(node_clique) // Pushes a Vec<NodeStats> onto another vector
    }
    Ok(node_cliques)
}
/// Manipulates the views field of NodeStats to output each node's % of the clique's total viewership count \
/// **Input**: vector of vectors containing NodeStats such as \[NodeStat1, NodeStats2, NodeStats3\] \
/// Sums the views field for each vector \
/// Divides each node's view field by vector's sum \
/// **Output:** vector of vectors containing tuples (node_id as u32, % of total viewership as f32)
pub fn viewership_distribution (cliques: &Vec<Vec<NodeStats>>) -> Vec<Vec<(u32, f32)>> {
    let mut all_view_dists = Vec::new();
    for clique in cliques {
        let sum = clique.iter().fold(0, |acc, node| acc + node.views);
        let mut clique_viewership_dist = Vec::new();
        for node in clique {
            let node_views_percent: f32 = node.views as f32 / sum as f32;
            clique_viewership_dist.push((node.new_id, node_views_percent));
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
    let true_dists: Vec<Vec<(u32, f32)>> = vec![
        vec![(1, 100.0/600.0), (2, 200.0/600.0), (3, 300.0/600.0)], 
        vec![(1, 100.0/400.0), (3, 300.0/400.0)]
        ];

    assert_eq!(dists, true_dists, "Distributions not aligning");

}


use plotters::prelude::*;
/// Uses plotters to generate barcharts of the distribution of viewership statistics for each clique \
/// **Input:** vector of vectors containing tuples (node_id as u32, % of total viewership as f32) \
/// Subdivides BitMap based on # of cliques \
/// *For clique in input_vector* { \
/// Creates bar chart for each clique's distribution \
/// } \
/// **Output:** bar charts outputted as "viewership_distributions.png" in project_code folder \
/// 
/// **Note** \
/// While BitMap generation and area subdivison are handled dynamically by the number of cliques,
/// the use of a single .png may be unwise for high numbers of cliques
pub fn plot_viewership_distributions (distributions: Vec<Vec<(u32, f32)>>) {
    
    let num_cliques: f32 = distributions.len() as f32;
    let mut sub_area_rows = num_cliques.sqrt().floor() as usize;
    let sub_area_cols = num_cliques.sqrt().ceil() as usize;
    if sub_area_rows * sub_area_cols < num_cliques as usize { //Ensures that there are enough slots for all clique graphs
        sub_area_rows += 1
    }
     
    let root_area = BitMapBackend::new("viewership_distributions.png"
    , ((sub_area_cols * 500) as u32, (sub_area_rows as f32 * 500.0 * 0.75).floor() as u32)) // Defines root_area dynamically to handle changes in # of cliques
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let sub_areas = root_area.split_evenly((sub_area_rows, sub_area_cols)); // Divides root_area up into spaces for each bar chart
    for ((idx, clique), area) in (1..).zip(distributions.iter()).zip(sub_areas) { // Create chart for each clique (each vector)
        let node_names: Vec<u32> = clique.iter().map(|node| node.0).collect(); // Collects node ids from tuple
        let y_values: Vec<f32> = clique.iter().map(|node| node.1).collect(); // Collects viewership % from tuple
        let mut chart = ChartBuilder::on(&area)  // Generate chart context, taken from lecture notes
            .caption(format!("Viewership Distribution for Clique {}", idx), ("Arial", 15).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d((0..(node_names.len() - 1)).into_segmented(), 0f32..1f32).unwrap();

        chart.configure_mesh() // Configure the chart labels and line thickness, referenced from lecture notes
            .y_labels(10)
            .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32)) // Reformat y values as %s
            .light_line_style(&TRANSPARENT)
            .x_desc("Nodes in Clique")
            .y_desc("% of Clique's Total Viewership")
            .draw()
            .unwrap();
        
        chart.draw_series(node_names.iter().enumerate().map(|(i, &_node_id)| { // Draw the rectangles on the chart, referenced from the Plotters Developer's Guide
            let x0 = SegmentValue::Exact(i);
            let x1 = SegmentValue::Exact(i + 1);
            Rectangle::new([(x0, 0f32), (x1, y_values[i])], RED.filled())
        }))
        .unwrap();

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .draw()
            .unwrap();
    }
    
}