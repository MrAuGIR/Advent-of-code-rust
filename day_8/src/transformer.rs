use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Lines};
use regex::Regex;

use crate::components::Node;

pub fn create_graph<'a>(lines: Lines<io::BufReader<File>>) -> HashMap<String, Node> {

    let mut graph :HashMap<String, Node> = HashMap::new();

    for line in lines {
        
        if let Ok(line_content) = line {
            let mut node_master = String::new();
            let mut node_neighbor_left = String::new();
            let mut node_neighbor_right = String::new();
            let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

            if let Some(captures) = re.captures(line_content.as_str()) {
                node_master = captures.get(1).unwrap().as_str().to_string();
                node_neighbor_left = captures.get(2).unwrap().as_str().to_string();
                node_neighbor_right = captures.get(3).unwrap().as_str().to_string();
            }

            graph.insert(
                node_master.clone(),
                Node::new(node_master, node_neighbor_left, node_neighbor_right),
            );
        }
    }

    graph.to_owned()
}


pub fn create_instructions<'a>(lines: Lines<io::BufReader<File>>) -> String {
    let mut suite_instruction = String::new();
    for line in lines {
        
        if let Ok(line_content) = line {

            suite_instruction = format!("{}{}",suite_instruction,line_content);

        }
    }
    suite_instruction
}