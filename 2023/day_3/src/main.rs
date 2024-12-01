mod components;
mod reader;
mod transformer;

use crate::reader::*;
use crate::transformer::*;
use crate::components::*;

fn main() {
    let path = String::from("./input/data.txt");

    // lecture du fichier
    if let Ok(lines) = read_lines(path.to_owned()) {

        let graph = transform_to_graph(lines);

     //    let nodes = graph.find_by_type(Type::PartNumber(000));  // premiere partie
     //    let sum: u32 = nodes.iter().map(|&node| node.node_type.get_integer_value().unwrap_or(0)).sum(); // premiere partie

        let nodes_gears = graph.find_gear(Type::Gear("x".to_string()));
        let sum: u32 = nodes_gears.iter().map(|&node| node.sum_real_gear(&graph)).sum();

    
        println!("{:#?}",sum);
    }   
}

